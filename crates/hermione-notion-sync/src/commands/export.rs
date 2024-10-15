use crate::{screen, settings::Settings, Result};
use hermione_coordinator::{
    commands::{self, Operations as _},
    workspaces::{self, Operations as _},
    Connection,
};
use hermione_notion::{
    json::{Json, PageId, RichText, Title},
    QueryDatabaseParameters,
};
use serde::Serialize;
use std::{path::PathBuf, rc::Rc};

const PAGE_SIZE: u32 = 100;

pub struct Command {
    settings: Settings,
    notion_client: hermione_notion::Client,
    workspaces_coordinator: workspaces::Client,
    commands_coordinator: commands::Client,
    total_workspaces: u32,
    created_workspaces: u32,
    updated_workspaces: u32,
    exported_workspaces: u32,
    total_commands: u32,
    created_commands: u32,
    updated_commands: u32,
    exported_commands: u32,
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

        self.created_commands += 1;
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

        self.created_workspaces += 1;
        Ok(())
    }

    pub async fn execute(mut self) -> Result<()> {
        self.export_workspaces().await?;
        self.export_commands().await?;

        screen::clear_and_reset_cursor();

        screen::print("Workspaces export summary:");
        screen::print(&format!("Total workspaces: {}", self.total_workspaces));
        screen::print(&format!("Created workspaces: {}", self.created_workspaces));
        screen::print(&format!("Updated workspaces: {}", self.updated_workspaces));

        screen::print("");

        screen::print("Commands export summary:");
        screen::print(&format!("Total commands: {}", self.total_commands));
        screen::print(&format!("Created commands: {}", self.created_commands));
        screen::print(&format!("Updated commands: {}", self.updated_commands));

        Ok(())
    }

    async fn export_commands(&mut self) -> Result<()> {
        let mut page_number = 0;

        loop {
            let local_commands = self.local_commands(page_number)?;
            self.total_commands += local_commands.len() as u32;

            if local_commands.is_empty() {
                return Ok(());
            }

            let remote_commands = self.remote_commands(&local_commands).await?;
            self.sync_commands(local_commands, remote_commands).await?;

            page_number += 1;
        }
    }

    async fn export_workspaces(&mut self) -> Result<()> {
        let mut page_number = 0;

        loop {
            let local_workspaces = self.local_workspaces(page_number)?;
            self.total_workspaces += local_workspaces.len() as u32;

            if local_workspaces.is_empty() {
                return Ok(());
            }

            let remote_workspaces = self.remote_workspaces(&local_workspaces).await?;
            self.sync_workspaces(local_workspaces, remote_workspaces)
                .await?;

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
            settings,
            notion_client,
            commands_coordinator,
            workspaces_coordinator,
            total_workspaces: 0,
            created_workspaces: 0,
            updated_workspaces: 0,
            exported_workspaces: 0,
            total_commands: 0,
            created_commands: 0,
            updated_commands: 0,
            exported_commands: 0,
        })
    }

    async fn remote_commands(&self, commands: &[commands::Dto]) -> Result<Vec<NotionCommand>> {
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

        let json = self
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

        let results = json["results"].as_array();

        let Some(results) = results else {
            return Ok(Vec::new());
        };

        let workspaces = results.iter().map(Into::into).collect();

        Ok(workspaces)
    }

    async fn remote_workspaces(
        &self,
        workspaces: &[workspaces::Dto],
    ) -> Result<Vec<NotionWorkspace>> {
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

        let json = self
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

        let results = json["results"].as_array();

        let Some(results) = results else {
            return Ok(Vec::new());
        };

        let workspaces = results.iter().map(Into::into).collect();

        Ok(workspaces)
    }

    async fn sync_commands(
        &mut self,
        local_commands: Vec<commands::Dto>,
        remote_commands: Vec<NotionCommand>,
    ) -> Result<()> {
        for local_command in local_commands {
            self.exported_commands += 1;

            screen::clear_and_reset_cursor();
            screen::print(&format!(
                "Syncing {}/{} commands...",
                self.exported_commands, self.total_commands,
            ));

            let remote_command = remote_commands
                .iter()
                .find(|remote_command| remote_command.external_id == local_command.id);

            if let Some(remote_command) = remote_command {
                if remote_command == &local_command {
                    continue;
                }

                self.update_remote_command(local_command, remote_command)
                    .await?;
            } else {
                self.create_remote_command(local_command).await?;
            }
        }

        Ok(())
    }

    async fn sync_workspaces(
        &mut self,
        local_workspaces: Vec<workspaces::Dto>,
        remote_workspaces: Vec<NotionWorkspace>,
    ) -> Result<()> {
        for local_workspace in local_workspaces {
            self.exported_workspaces += 1;

            screen::clear_and_reset_cursor();
            screen::print(&format!(
                "Syncing {}/{} workspaces...",
                self.exported_workspaces, self.total_workspaces,
            ));

            let remote_workspace = remote_workspaces
                .iter()
                .find(|remote_workspace| remote_workspace.external_id == local_workspace.id);

            if let Some(remote_workspace) = remote_workspace {
                if remote_workspace == &local_workspace {
                    continue;
                }

                self.update_remote_workspace(local_workspace, remote_workspace)
                    .await?;
            } else {
                self.create_remote_workspace(local_workspace).await?;
            }
        }

        Ok(())
    }

    async fn update_remote_command(
        &mut self,
        local_command: commands::Dto,
        remote_command: &NotionCommand,
    ) -> Result<()> {
        self.notion_client
            .update_database_entry(
                &remote_command.id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_command.name}}]},
                    "Program": {"rich_text": [{"text": {"content": local_command.program}}]}
                }),
            )
            .await?;

        self.updated_commands += 1;

        Ok(())
    }

    async fn update_remote_workspace(
        &mut self,
        local_workspace: workspaces::Dto,
        remote_workspace: &NotionWorkspace,
    ) -> Result<()> {
        self.notion_client
            .update_database_entry(
                &remote_workspace.id,
                serde_json::json!({
                    "Name": {"title": [{"text": {"content": local_workspace.name}}]},
                    "Location": {"rich_text": [{"text": {"content": local_workspace.location}}]}
                }),
            )
            .await?;

        self.updated_workspaces += 1;

        Ok(())
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
