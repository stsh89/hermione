use crate::{
    screen,
    settings::Settings,
    statistics::{Action, Statistics},
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
use hermione_notion::{
    json::{Json, PageId, RichText, Title},
    QueryDatabaseParameters,
};
use serde::Serialize;
use std::{path::PathBuf, rc::Rc};

const PAGE_SIZE: u8 = 1;

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

#[derive(Debug)]
struct NotionWorkspace {
    external_id: String,
    location: String,
    name: String,
    id: String,
}

#[derive(Debug)]
struct NotionCommand {
    external_id: String,
    id: String,
    name: String,
    program: String,
    workspace_id: String,
}

impl Command {
    async fn create_local_command(&mut self, remote_command: NotionCommand) -> Result<()> {
        self.commands_coordinator.import(commands::Dto {
            id: remote_command.external_id,
            last_execute_time: None,
            name: remote_command.name,
            program: remote_command.program,
            workspace_id: remote_command.workspace_id,
        })?;

        self.commands_statistics.track_action(Action::Create);

        Ok(())
    }

    async fn create_local_workspace(&mut self, remote_workspace: NotionWorkspace) -> Result<()> {
        self.workspaces_coordinator.import(workspaces::Dto {
            id: remote_workspace.external_id,
            last_access_time: None,
            location: Some(remote_workspace.location),
            name: remote_workspace.name,
        })?;

        self.workspaces_statistics.track_action(Action::Create);

        Ok(())
    }

    pub async fn execute(mut self) -> Result<()> {
        self.import_workspaces().await?;
        self.import_commands().await?;

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

    async fn import_commands(&mut self) -> Result<()> {
        let (mut remote_commands, mut pagination_token) = self.remote_commands(None).await?;
        self.import_commands_batch(remote_commands).await?;

        while pagination_token.is_some() {
            (remote_commands, pagination_token) =
                self.remote_commands(pagination_token.as_deref()).await?;

            self.import_commands_batch(remote_commands).await?;
        }

        Ok(())
    }

    async fn import_workspaces(&mut self) -> Result<()> {
        let (mut remote_workspaces, mut pagination_token) = self.remote_workspaces(None).await?;
        self.import_workspaces_batch(remote_workspaces).await?;

        while pagination_token.is_some() {
            (remote_workspaces, pagination_token) =
                self.remote_workspaces(pagination_token.as_deref()).await?;

            self.import_workspaces_batch(remote_workspaces).await?;
        }

        Ok(())
    }

    async fn remote_commands(
        &self,
        pagination_token: Option<&str>,
    ) -> Result<(Vec<NotionCommand>, Option<String>)> {
        let json = self
            .notion_client
            .query_database(
                self.settings.commands_page_id(),
                QueryDatabaseParameters {
                    page_size: PAGE_SIZE,
                    start_cursor: pagination_token.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

        let pagination_token = json["next_cursor"].as_str().map(Into::into);
        let results = json["results"].as_array();

        let Some(results) = results else {
            return Ok((Vec::new(), None));
        };

        let commands = results.iter().map(Into::into).collect();

        Ok((commands, pagination_token))
    }

    async fn remote_workspaces(
        &self,
        pagination_token: Option<&str>,
    ) -> Result<(Vec<NotionWorkspace>, Option<String>)> {
        let json = self
            .notion_client
            .query_database(
                self.settings.workspaces_page_id(),
                QueryDatabaseParameters {
                    page_size: PAGE_SIZE,
                    start_cursor: pagination_token.as_deref(),
                    ..Default::default()
                },
            )
            .await?;

        let pagination_token = json["next_cursor"].as_str().map(Into::into);
        let results = json["results"].as_array();

        let Some(results) = results else {
            return Ok((Vec::new(), None));
        };

        let commands = results.iter().map(Into::into).collect();

        Ok((commands, pagination_token))
    }

    pub fn new(directory_path: PathBuf) -> Result<Self> {
        let settings = Settings::read(&directory_path)?;

        let notion_client = hermione_notion::Client::new(hermione_notion::NewClientParameters {
            api_key: Some(settings.api_key().into()),
            ..Default::default()
        })?;

        let connection = Rc::new(Connection::open(&directory_path)?);
        let workspaces_coordinator = workspaces::Client::new(connection.clone());
        let commands_coordinator = commands::Client::new(connection.clone());

        Ok(Self {
            commands_coordinator,
            commands_statistics: Statistics::default(),
            notion_client,
            settings,
            workspaces_coordinator,
            workspaces_statistics: Statistics::default(),
        })
    }

    async fn import_commands_batch(&mut self, remote_commands: Vec<NotionCommand>) -> Result<()> {
        for remote_command in remote_commands {
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

    async fn import_workspaces_batch(
        &mut self,
        remote_workspaces: Vec<NotionWorkspace>,
    ) -> Result<()> {
        for remote_workspace in remote_workspaces {
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

    async fn update_local_command(&mut self, remote_command: NotionCommand) -> Result<()> {
        self.commands_coordinator.update(commands::Dto {
            id: remote_command.external_id,
            last_execute_time: None,
            name: remote_command.name,
            program: remote_command.program,
            workspace_id: remote_command.workspace_id,
        })?;

        self.commands_statistics.track_action(Action::Update);

        Ok(())
    }

    async fn update_local_workspace(&mut self, remote_workspace: NotionWorkspace) -> Result<()> {
        self.workspaces_coordinator.update(workspaces::Dto {
            id: remote_workspace.external_id,
            last_access_time: None,
            location: Some(remote_workspace.location),
            name: remote_workspace.name,
        })?;

        self.workspaces_statistics.track_action(Action::Update);

        Ok(())
    }

    fn verify_local_command(
        &mut self,
        remote_command: &NotionCommand,
        local_command: &commands::Dto,
    ) -> bool {
        let verified = remote_command == local_command;

        if verified {
            self.commands_statistics.track_action(Action::Verify);
        }

        verified
    }

    fn verify_local_workspace(
        &mut self,
        remote_workspace: &NotionWorkspace,
        local_workspace: &workspaces::Dto,
    ) -> bool {
        let verified = remote_workspace == local_workspace;

        if verified {
            self.workspaces_statistics.track_action(Action::Verify);
        }

        verified
    }
}

impl From<&Json> for NotionWorkspace {
    fn from(json: &Json) -> Self {
        NotionWorkspace {
            id: json.id().into(),
            external_id: json.rich_text("External ID").into(),
            location: json.rich_text("Location").into(),
            name: json.title().into(),
        }
    }
}

impl From<&Json> for NotionCommand {
    fn from(json: &Json) -> Self {
        NotionCommand {
            id: json.id().into(),
            external_id: json.rich_text("External ID").into(),
            program: json.rich_text("Program").into(),
            name: json.title().into(),
            workspace_id: json.rich_text("Workspace ID").into(),
        }
    }
}

impl PartialEq<workspaces::Dto> for NotionWorkspace {
    fn eq(&self, other: &workspaces::Dto) -> bool {
        self.name == other.name && self.location == other.location.as_deref().unwrap_or_default()
    }
}

impl PartialEq<commands::Dto> for NotionCommand {
    fn eq(&self, other: &commands::Dto) -> bool {
        self.name == other.name && self.program == other.program
    }
}
