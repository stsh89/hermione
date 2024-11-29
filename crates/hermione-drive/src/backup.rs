use eyre::Report;
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceId, WorkspaceParameters,
    },
    services::{
        BackupCommands, BackupCopies, BackupCopyParameters, BackupService, BackupServiceBuilder,
        BackupWorkspaces, GetCommandsBackupCopy, GetWorkspacesBackupCopy, VerifyBackupCredentials,
    },
    Error, Result,
};
use rusty_notion::api::{
    self, Client, CreateDatabaseEntryParameters, QueryDatabaseParameters,
    UpdateDatabaseEntryParameters,
};
use std::{num::NonZeroU32, thread};
use ureq::Response;
use uuid::Uuid;

const DEFAULT_BACKUP_PAGE_SIZE: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100) };

use hermione_internals::notion::{
    self, external_ids_filter, verify_commands_database_properties,
    verify_workspaces_database_properties, NotionCommandProperties, NotionWorkspaceProperties,
    QueryDatabaseResponse,
};

pub struct NotionBackup {
    client: Client,
    commands_database_id: String,
    page_size: NonZeroU32,
    workspaces_database_id: String,
}

struct NotionBackupParameters {
    credentials: NotionBackupCredentials,
    page_size: NonZeroU32,
}

#[derive(Default)]
pub struct NotionBackupBuilder {
    pub page_size: Option<NonZeroU32>,
}

impl NotionBackup {
    fn new(parameters: NotionBackupParameters) -> Result<Self> {
        let NotionBackupParameters {
            credentials,
            page_size,
        } = parameters;

        let api_client = Client::new(credentials.api_key().to_string());

        Ok(Self {
            client: api_client,
            commands_database_id: credentials.commands_database_id().to_string(),
            page_size,
            workspaces_database_id: credentials.workspaces_database_id().to_string(),
        })
    }
}

impl NotionBackupBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> Result<NotionBackup> {
        let BackupCredentials::Notion(credentials) = credentials;
        let page_size = self.page_size.unwrap_or(DEFAULT_BACKUP_PAGE_SIZE);

        NotionBackup::new(NotionBackupParameters {
            credentials,
            page_size,
        })
    }
}

impl BackupServiceBuilder<NotionBackup> for NotionBackupBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<NotionBackup> {
        self.build(credentials.clone())
    }
}

impl BackupService for NotionBackup {}

impl GetCommandsBackupCopy for NotionBackup {
    fn get_commands_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Command>> {
        let BackupCopyParameters { page_token } = parameters;

        let query_database = || {
            api::query_database(
                &self.client,
                QueryDatabaseParameters {
                    database_id: &self.commands_database_id,
                    start_cursor: page_token,
                    page_size: Some(self.page_size),
                    filter: None,
                },
            )
        };

        let response = send_with_retries(query_database)?;

        let database_query_response: QueryDatabaseResponse<NotionCommandProperties> =
            notion::query_datrabase_response(response)
                .map_err(|err| {
                    err.wrap_err(
                        "Could not process Notion API response. API: query commands database",
                    )
                })
                .map_err(Error::backup)?;

        let next_page_token = database_query_response.next_cursor;

        let commands = database_query_response
            .database_pages
            .into_iter()
            .map(|page| {
                let id = page
                    .properties
                    .external_id
                    .parse()
                    .map_err(|err| {
                        Report::new(err).wrap_err(format!(
                            "Invalid backup data. Could not parse command ID: {}",
                            page.properties.external_id
                        ))
                    })
                    .map_err(Error::backup)?;

                let workspace_id: Uuid = page
                    .properties
                    .workspace_id
                    .parse()
                    .map_err(|err| {
                        Report::new(err).wrap_err(format!(
                            "Invalid backup data. Could not parse command's workspace ID: {}",
                            page.properties.workspace_id
                        ))
                    })
                    .map_err(Error::backup)?;

                Command::new(CommandParameters {
                    id,
                    last_execute_time: None,
                    program: page.properties.program,
                    name: page.properties.name,
                    workspace_id: WorkspaceId::new(workspace_id)?,
                })
            })
            .collect::<Result<Vec<Command>>>()?;

        Ok(BackupCopies {
            copies: commands,
            next_page_token,
        })
    }
}

impl GetWorkspacesBackupCopy for NotionBackup {
    fn get_workspaces_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Workspace>> {
        let BackupCopyParameters { page_token } = parameters;

        let query_database = || {
            api::query_database(
                &self.client,
                QueryDatabaseParameters {
                    database_id: &self.workspaces_database_id,
                    start_cursor: page_token,
                    page_size: Some(self.page_size),
                    filter: None,
                },
            )
        };

        let response = send_with_retries(query_database)?;

        let database_query_response: QueryDatabaseResponse<NotionWorkspaceProperties> =
            notion::query_datrabase_response(response)
                .map_err(|err| {
                    err.wrap_err(
                        "Could not process Notion API response. API: query workspaces database",
                    )
                })
                .map_err(Error::backup)?;

        let next_page_token = database_query_response.next_cursor;

        let workspaces = database_query_response
            .database_pages
            .into_iter()
            .map(|page| {
                let id = page
                    .properties
                    .external_id
                    .parse()
                    .map_err(|err| {
                        Report::new(err).wrap_err(format!(
                            "Invalid backup data. Could not parse workspace ID: {}",
                            page.properties.external_id
                        ))
                    })
                    .map_err(Error::backup)?;

                Workspace::new(WorkspaceParameters {
                    id,
                    last_access_time: None,
                    location: Some(page.properties.location),
                    name: page.properties.name,
                })
            })
            .collect::<Result<Vec<Workspace>>>()?;

        Ok(BackupCopies {
            copies: workspaces,
            next_page_token,
        })
    }
}

impl BackupCommands for NotionBackup {
    fn backup_commands(&self, commands: Vec<Command>) -> Result<()> {
        if commands.is_empty() {
            return Ok(());
        }

        let external_ids: Vec<String> = commands
            .iter()
            .map(|command| command.id().to_string())
            .collect();
        let count = external_ids.len();

        let filter = external_ids_filter(external_ids);

        let query_database = || {
            api::query_database(
                &self.client,
                QueryDatabaseParameters {
                    database_id: &self.commands_database_id,
                    start_cursor: None,
                    page_size: NonZeroU32::new(count as u32),
                    filter: filter.clone(),
                },
            )
        };

        let response = send_with_retries(query_database)?;

        let response: QueryDatabaseResponse<NotionCommandProperties> =
            notion::query_datrabase_response(response)
                .map_err(|err| {
                    err.wrap_err(
                        "Could not process Notion API response. API: query commands database",
                    )
                })
                .map_err(Error::backup)?;

        for command in commands {
            let page = response
                .database_pages
                .iter()
                .find(|p| p.properties.external_id == command.id().to_string());

            let Some(page) = page else {
                let api_call = || {
                    api::create_database_entry(
                        &self.client,
                        CreateDatabaseEntryParameters {
                            database_id: &self.commands_database_id,
                            properties: serde_json::json!({
                                "Name": {"title": [{"text": {"content": command.name()}}]},
                                "External ID": {"rich_text": [{"text": {"content": command.id().to_string()}}]},
                                "Program": {"rich_text": [{"text": {"content": command.program()}}]},
                                "Workspace ID": {"rich_text": [{"text": {"content": command.workspace_id().to_string()}}]}
                            }),
                        },
                    )
                };

                send_with_retries(api_call)?;

                continue;
            };

            if command.name() != page.properties.name
                || command.program() != page.properties.program
            {
                let api_call = || {
                    api::update_database_entry(
                        &self.client,
                        UpdateDatabaseEntryParameters {
                            entry_id: &page.page_id,
                            properties: serde_json::json!({
                                "Name": {"title": [{"text": {"content": command.name()}}]},
                                "Program": {"rich_text": [{"text": {"content": command.program()}}]}
                            }),
                        },
                    )
                };

                send_with_retries(api_call)?;
            }
        }

        Ok(())
    }
}

impl BackupWorkspaces for NotionBackup {
    fn backup_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()> {
        if workspaces.is_empty() {
            return Ok(());
        }

        let external_ids: Vec<String> = workspaces
            .iter()
            .map(|command| command.id().to_string())
            .collect();
        let count = external_ids.len();

        let filter = external_ids_filter(external_ids);

        let api_call = || {
            api::query_database(
                &self.client,
                QueryDatabaseParameters {
                    database_id: &self.workspaces_database_id,
                    start_cursor: None,
                    page_size: NonZeroU32::new(count as u32),
                    filter: filter.clone(),
                },
            )
        };

        let response = send_with_retries(api_call)?;

        let response: QueryDatabaseResponse<NotionWorkspaceProperties> =
            notion::query_datrabase_response(response)
                .map_err(|err| {
                    err.wrap_err(
                        "Could not process Notion API response. API: query workspaces database",
                    )
                })
                .map_err(Error::backup)?;

        for workspace in workspaces {
            let page = response
                .database_pages
                .iter()
                .find(|p| p.properties.external_id == workspace.id().to_string());

            let Some(page) = page else {
                let api_call = || {
                    api::create_database_entry(
                        &self.client,
                        CreateDatabaseEntryParameters {
                            database_id: &self.workspaces_database_id,
                            properties: serde_json::json!({
                                "Name": {"title": [{"text": {"content": workspace.name()}}]},
                                "External ID": {"rich_text": [{"text": {"content": workspace.id().to_string()}}]},
                                "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                            }),
                        },
                    )
                };

                send_with_retries(api_call)?;

                continue;
            };

            if workspace.name() != page.properties.name
                || workspace.location().unwrap_or_default() != page.properties.location
            {
                let api_call = || {
                    api::update_database_entry(
                        &self.client,
                        UpdateDatabaseEntryParameters {
                            entry_id: &page.page_id,
                            properties: serde_json::json!({
                                "Name": {"title": [{"text": {"content": workspace.name()}}]},
                                "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                            }),
                        },
                    )
                };

                send_with_retries(api_call)?;
            }
        }

        Ok(())
    }
}

impl VerifyBackupCredentials for NotionBackup {
    fn verify_backup_credentials(&self) -> Result<()> {
        let response = api::query_database_properties(&self.client, &self.commands_database_id)
            .map_err(|err| {
                Report::new(err).wrap_err("Failed to get Notion commands database properties")
            })
            .map_err(Error::backup)?;

        let properties = notion::get_database_properties(response)
            .map_err(|err| {
                err.wrap_err("Could not process Notion API response. API: query commands database")
            })
            .map_err(Error::backup)?;

        verify_commands_database_properties(properties)
            .map_err(|err| err.wrap_err("Incorrect Notion commands database properties"))
            .map_err(Error::backup)?;

        let response = api::query_database_properties(&self.client, &self.workspaces_database_id)
            .map_err(|err| {
                Report::new(err).wrap_err("Failed to get Notion commands database properties")
            })
            .map_err(Error::backup)?;

        let properties = notion::get_database_properties(response)
            .map_err(|err| {
                err.wrap_err(
                    "Could not process Notion API response. API: query workspaces database",
                )
            })
            .map_err(Error::backup)?;

        verify_workspaces_database_properties(properties)
            .map_err(|err| err.wrap_err("Incorrect Notion workspace database properties"))
            .map_err(Error::backup)?;

        Ok(())
    }
}

fn send_with_retries(f: impl Fn() -> api::Result<Response>) -> Result<Response> {
    api::send_with_retries(f, thread::sleep)
        .map_err(|err| Report::new(err).wrap_err("Notion API request failure"))
        .map_err(Error::backup)
}
