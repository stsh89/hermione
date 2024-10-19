use crate::{
    notion::{Action, Command, Statistics, Workspace},
    screen,
    settings::Settings,
    Result,
};
use hermione_coordinator::{
    workspaces::{
        self,
        commands::{self, Operations as _},
        Operations as _,
    },
    Connection,
};
use hermione_notion::{DatabasePage, QueryDatabaseParameters, QueryDatabaseResponse};
use serde::Serialize;
use std::{path::Path, rc::Rc};

const PAGE_SIZE: u8 = 100;

pub struct Operation {
    settings: Settings,
    notion_client: hermione_notion::Client,
    workspaces_coordinator: workspaces::Client,
    commands_coordinator: commands::Client,
    workspaces_statistics: Statistics,
    commands_statistics: Statistics,
}

impl crate::Operation for Operation {
    fn execute(&self) -> crate::OperationResult {
        Box::pin(self.run())
    }
}

#[derive(Serialize)]
struct StatisticsSummary {
    commands: Summary,
    workspaces: Summary,
}

#[derive(Serialize)]
struct Summary {
    created: u32,
    total: u32,
    updated: u32,
}

impl Operation {
    async fn create_local_command(&self, remote_command: Command) -> Result<()> {
        self.commands_coordinator.import(remote_command.into())?;
        self.commands_statistics.track(Action::Create);

        Ok(())
    }

    async fn create_local_workspace(&self, remote_workspace: Workspace) -> Result<()> {
        self.workspaces_coordinator
            .import(remote_workspace.into())?;
        self.workspaces_statistics.track(Action::Create);

        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        self.import_workspaces().await?;
        self.import_commands().await?;

        screen::clear_and_reset_cursor();
        self.print_statistics_summary();

        Ok(())
    }

    fn print_statistics_summary(&self) {
        let summary = StatisticsSummary {
            workspaces: Summary {
                created: self.workspaces_statistics.count(Action::Create),
                total: self.workspaces_statistics.total(),
                updated: self.workspaces_statistics.count(Action::Update),
            },
            commands: Summary {
                created: self.commands_statistics.count(Action::Create),
                total: self.commands_statistics.total(),
                updated: self.commands_statistics.count(Action::Update),
            },
        };

        screen::print(&serde_json::to_string(&summary).unwrap_or_default());
    }

    async fn import_commands(&self) -> Result<()> {
        let mut query_database_response = self.remote_commands(None).await?;
        self.import_commands_batch(query_database_response.database_pages)
            .await?;

        while query_database_response.next_cursor.is_some() {
            query_database_response = self
                .remote_commands(query_database_response.next_cursor.as_deref())
                .await?;

            self.import_commands_batch(query_database_response.database_pages)
                .await?;
        }

        Ok(())
    }

    async fn import_workspaces(&self) -> Result<()> {
        let mut query_database_response = self.remote_workspaces(None).await?;
        self.import_workspaces_batch(query_database_response.database_pages)
            .await?;

        while query_database_response.next_cursor.is_some() {
            query_database_response = self
                .remote_workspaces(query_database_response.next_cursor.as_deref())
                .await?;

            self.import_workspaces_batch(query_database_response.database_pages)
                .await?;
        }

        Ok(())
    }

    async fn remote_commands(
        &self,
        start_cursor: Option<&str>,
    ) -> Result<QueryDatabaseResponse<Command>> {
        let query_database_response = self
            .notion_client
            .query_database(
                self.settings.commands_page_id(),
                QueryDatabaseParameters {
                    page_size: PAGE_SIZE,
                    start_cursor,
                    ..Default::default()
                },
            )
            .await?;

        Ok(query_database_response)
    }

    async fn remote_workspaces(
        &self,
        start_cursor: Option<&str>,
    ) -> Result<QueryDatabaseResponse<Workspace>> {
        let query_database_response = self
            .notion_client
            .query_database(
                self.settings.workspaces_page_id(),
                QueryDatabaseParameters {
                    page_size: PAGE_SIZE,
                    start_cursor,
                    ..Default::default()
                },
            )
            .await?;

        Ok(query_database_response)
    }

    pub fn new(directory_path: &Path) -> Result<Self> {
        let settings = Settings::read(directory_path)?;

        let notion_client = hermione_notion::Client::new(hermione_notion::NewClientParameters {
            api_key: Some(settings.api_key().into()),
            ..Default::default()
        })?;

        let connection = Rc::new(Connection::open(directory_path)?);
        let workspaces_coordinator = workspaces::Client::new(connection.clone());
        let commands_coordinator = commands::Client::new(connection.clone());

        Ok(Self {
            commands_coordinator,
            commands_statistics: Statistics::new(),
            notion_client,
            settings,
            workspaces_coordinator,
            workspaces_statistics: Statistics::new(),
        })
    }

    async fn import_commands_batch(&self, pages: Vec<DatabasePage<Command>>) -> Result<()> {
        for page in pages {
            let DatabasePage {
                page_id: _,
                properties: remote_command,
            } = page;

            screen::clear_and_reset_cursor();
            screen::print("Importing commands from Notion...");

            let local_command = self
                .commands_coordinator
                .find(&remote_command.workspace_id, &remote_command.external_id)?;

            let Some(local_command) = local_command else {
                self.create_local_command(remote_command).await?;

                continue;
            };

            if !self.verify_local_command(&remote_command, &local_command) {
                self.update_local_command(remote_command).await?;
            }

            screen::print(&format!(
                "Imported {} commands...",
                self.commands_statistics.total()
            ));
        }

        Ok(())
    }

    async fn import_workspaces_batch(&self, pages: Vec<DatabasePage<Workspace>>) -> Result<()> {
        for page in pages {
            let DatabasePage {
                page_id: _,
                properties: remote_workspace,
            } = page;

            screen::clear_and_reset_cursor();
            screen::print("Importing workspaces from Notion...");

            let local_workspace = self
                .workspaces_coordinator
                .find(&remote_workspace.external_id)?;

            let Some(local_workspace) = local_workspace else {
                self.create_local_workspace(remote_workspace).await?;

                continue;
            };

            if !self.verify_local_workspace(&remote_workspace, &local_workspace) {
                self.update_local_workspace(remote_workspace).await?;
            }

            screen::print(&format!(
                "Imported {} workspaces...",
                self.workspaces_statistics.total()
            ));
        }

        Ok(())
    }

    async fn update_local_command(&self, remote_command: Command) -> Result<()> {
        self.commands_coordinator.update(remote_command.into())?;
        self.commands_statistics.track(Action::Update);

        Ok(())
    }

    async fn update_local_workspace(&self, remote_workspace: Workspace) -> Result<()> {
        self.workspaces_coordinator
            .update(remote_workspace.into())?;
        self.workspaces_statistics.track(Action::Update);

        Ok(())
    }

    fn verify_local_command(
        &self,
        remote_command: &Command,
        local_command: &commands::Dto,
    ) -> bool {
        let verified = remote_command == local_command;

        if verified {
            self.commands_statistics.track(Action::Verify);
        }

        verified
    }

    fn verify_local_workspace(
        &self,
        remote_workspace: &Workspace,
        local_workspace: &workspaces::Dto,
    ) -> bool {
        let verified = remote_workspace == local_workspace;

        if verified {
            self.workspaces_statistics.track(Action::Verify);
        }

        verified
    }
}
