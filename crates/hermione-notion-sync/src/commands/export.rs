use crate::{
    notion, screen,
    settings::Settings,
    statistics::{Action, Statistics},
    Result,
};
use hermione_coordinator::{
    commands::{self, Operations as _},
    workspaces::{self, Operations as _},
    Connection,
};
use hermione_notion::{DatabasePage, QueryDatabaseParameters, QueryDatabaseResponse};
use serde::Serialize;
use std::{path::PathBuf, rc::Rc};

const PAGE_SIZE: u32 = 100;

pub struct Command {
    settings: Settings,
    notion_client: hermione_notion::Client,
    workspaces_coordinator: workspaces::Client,
    commands_coordinator: commands::Client,
    workspaces_statistics: Statistics,
    commands_statistics: Statistics,
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

#[derive(Serialize)]
struct RichTextFilter {
    property: String,
    rich_text: RichTextEqualsFilter,
}

#[derive(Serialize)]
struct RichTextEqualsFilter {
    equals: String,
}

impl Command {
    async fn create_remote_command(&mut self, local_command: commands::Dto) -> Result<()> {
        self.notion_client
            .create_database_entry(
                self.settings.commands_page_id(),
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_command.name}}]},
                    "External ID": {"rich_text": [{"text": {"content": local_command.id}}]},
                    "Program": {"rich_text": [{"text": {"content": local_command.program}}]},
                    "Workspace ID": {"rich_text": [{"text": {"content": local_command.workspace_id}}]}
                }),
            )
            .await?;

        self.commands_statistics.track_action(Action::Create);

        Ok(())
    }

    async fn create_remote_workspace(&mut self, local_workspace: workspaces::Dto) -> Result<()> {
        self.notion_client
            .create_database_entry(
                self.settings.workspaces_page_id(),
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_workspace.name}}]},
                    "External ID": {"rich_text": [{"text": {"content": local_workspace.id}}]},
                    "Location": {"rich_text": [{"text": {"content": local_workspace.location}}]}
                }),
            )
            .await?;

        self.workspaces_statistics.track_action(Action::Create);

        Ok(())
    }

    pub async fn execute(mut self) -> Result<()> {
        self.export_workspaces().await?;
        self.export_commands().await?;

        screen::clear_and_reset_cursor();
        self.print_statistics_summary();

        Ok(())
    }

    fn print_statistics_summary(&self) {
        let summary = StatisticsSummary {
            workspaces: Summary {
                created: self.workspaces_statistics.counter(Action::Create),
                total: self.workspaces_statistics.total(),
                updated: self.workspaces_statistics.counter(Action::Update),
            },
            commands: Summary {
                created: self.commands_statistics.counter(Action::Create),
                total: self.commands_statistics.total(),
                updated: self.commands_statistics.counter(Action::Update),
            },
        };

        screen::print(&serde_json::to_string_pretty(&summary).unwrap_or_default());
    }

    async fn export_commands(&mut self) -> Result<()> {
        let mut page_number = 0;

        loop {
            let local_commands = self.local_commands(page_number)?;

            if local_commands.is_empty() {
                return Ok(());
            }

            self.export_commands_batch(local_commands).await?;

            page_number += 1;
        }
    }

    async fn export_workspaces(&mut self) -> Result<()> {
        let mut page_number = 0;

        loop {
            let local_workspaces = self.local_workspaces(page_number)?;

            if local_workspaces.is_empty() {
                return Ok(());
            }

            self.export_workspaces_batch(local_workspaces).await?;

            page_number += 1;
        }
    }

    fn local_commands(&self, page_number: u32) -> Result<Vec<commands::Dto>> {
        let commands = self.commands_coordinator.list(commands::ListParameters {
            page_number,
            page_size: PAGE_SIZE,
        })?;

        Ok(commands)
    }

    fn local_workspaces(&self, page_number: u32) -> Result<Vec<workspaces::Dto>> {
        let workspaces = self
            .workspaces_coordinator
            .list(workspaces::ListParameters {
                name_contains: "",
                page_number,
                page_size: PAGE_SIZE,
            })?;

        Ok(workspaces)
    }

    pub fn new(directory_path: PathBuf) -> Result<Self> {
        let settings = Settings::read(&directory_path)?;

        let notion_client = hermione_notion::Client::new(hermione_notion::NewClientParameters {
            api_key: Some(settings.api_key().into()),
            ..Default::default()
        })?;

        let connection = Rc::new(Connection::open(&directory_path)?);
        let workspaces_coordinator = workspaces::Client::new(connection.clone());
        let commands_coordinator = commands::Client::new(connection);

        Ok(Self {
            commands_coordinator,
            commands_statistics: Statistics::default(),
            notion_client,
            settings,
            workspaces_coordinator,
            workspaces_statistics: Statistics::default(),
        })
    }

    async fn remote_commands(
        &self,
        commands: &[commands::Dto],
    ) -> Result<QueryDatabaseResponse<notion::Command>> {
        let filters: Vec<RichTextFilter> = commands
            .iter()
            .map(|commands| RichTextFilter {
                property: "External ID".to_string(),
                rich_text: RichTextEqualsFilter {
                    equals: commands.id.clone(),
                },
            })
            .collect();

        let filter = serde_json::json!({
            "or": serde_json::json!(filters),
        });

        let query_database_response = self
            .notion_client
            .query_database(
                self.settings.commands_page_id(),
                QueryDatabaseParameters {
                    page_size: commands.len() as u8,
                    filter: Some(filter),
                    ..Default::default()
                },
            )
            .await?;

        Ok(query_database_response)
    }

    async fn remote_workspaces(
        &self,
        workspaces: &[workspaces::Dto],
    ) -> Result<QueryDatabaseResponse<notion::Workspace>> {
        let filters: Vec<RichTextFilter> = workspaces
            .iter()
            .map(|workspace| RichTextFilter {
                property: "External ID".to_string(),
                rich_text: RichTextEqualsFilter {
                    equals: workspace.id.clone(),
                },
            })
            .collect();

        let filter = serde_json::json!({
            "or": serde_json::json!(filters),
        });

        let query_database_response = self
            .notion_client
            .query_database(
                self.settings.workspaces_page_id(),
                QueryDatabaseParameters {
                    page_size: workspaces.len() as u8,
                    filter: Some(filter),
                    ..Default::default()
                },
            )
            .await?;

        Ok(query_database_response)
    }

    async fn export_commands_batch(&mut self, local_commands: Vec<commands::Dto>) -> Result<()> {
        let query_database_response = self.remote_commands(&local_commands).await?;

        for local_command in local_commands {
            screen::clear_and_reset_cursor();
            screen::print("Exporting commands to Notion...");

            let page = query_database_response
                .database_pages
                .iter()
                .find(|page| page.properties.external_id == local_command.id);

            let Some(page) = page else {
                self.create_remote_command(local_command).await?;

                continue;
            };

            if !self.verify_remote_command(&local_command, &page.properties) {
                self.update_remote_command(local_command, page).await?;
            }

            screen::print(&format!(
                "Exported {} commands...",
                self.commands_statistics.total()
            ));
        }

        Ok(())
    }

    async fn export_workspaces_batch(
        &mut self,
        local_workspaces: Vec<workspaces::Dto>,
    ) -> Result<()> {
        let remote_workspaces = self.remote_workspaces(&local_workspaces).await?;

        for local_workspace in local_workspaces {
            screen::clear_and_reset_cursor();
            screen::print("Exporting workspaces to Notion...");

            let page = remote_workspaces
                .database_pages
                .iter()
                .find(|page| page.properties.external_id == local_workspace.id);

            let Some(page) = page else {
                self.create_remote_workspace(local_workspace).await?;

                continue;
            };

            if !self.verify_remote_workspace(&local_workspace, &page.properties) {
                self.update_remote_workspace(local_workspace, page).await?;
            }

            screen::print(&format!(
                "Exported {} workspaces...",
                self.workspaces_statistics.total()
            ));
        }

        Ok(())
    }

    async fn update_remote_command(
        &mut self,
        local_command: commands::Dto,
        database_page: &DatabasePage<notion::Command>,
    ) -> Result<()> {
        self.notion_client
            .update_database_entry(
                &database_page.page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_command.name}}]},
                    "Program": {"rich_text": [{"text": {"content": local_command.program}}]}
                }),
            )
            .await?;

        self.commands_statistics.track_action(Action::Update);

        Ok(())
    }

    async fn update_remote_workspace(
        &mut self,
        local_workspace: workspaces::Dto,
        database_page: &DatabasePage<notion::Workspace>,
    ) -> Result<()> {
        self.notion_client
            .update_database_entry(
                &database_page.page_id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_workspace.name}}]},
                    "Location": {"rich_text": [{"text": {"content": local_workspace.location}}]}
                }),
            )
            .await?;

        self.workspaces_statistics.track_action(Action::Update);

        Ok(())
    }

    fn verify_remote_command(
        &mut self,
        local_command: &commands::Dto,
        remote_command: &notion::Command,
    ) -> bool {
        let verified = remote_command == local_command;

        if verified {
            self.commands_statistics.track_action(Action::Verify);
        }

        verified
    }

    fn verify_remote_workspace(
        &mut self,
        local_workspace: &workspaces::Dto,
        remote_workspace: &notion::Workspace,
    ) -> bool {
        let verified = remote_workspace == local_workspace;

        if verified {
            self.workspaces_statistics.track_action(Action::Verify);
        }

        verified
    }
}
