use hermione_drive::notion::{
    external_ids_filter, get_database_properties, query_datrabase_response,
    verify_commands_database_properties, verify_workspaces_database_properties,
    CreateDatabaseEntryParameters, NotionApiClient, NotionApiClientParameters,
    NotionCommandProperties, NotionWorkspaceProperties, QueryDatabaseParameters,
    QueryDatabaseResponse, UpdateDatabaseEntryParameters,
};
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceParameters,
    },
    services::{
        BackupService, BackupServiceBuilder, ListCommandsBackup, ListWorkspacesBackup,
        UpsertCommandsBackup, UpsertWorkspacesBackup, VerifyBackupCredentials,
    },
    Error, Result,
};
use std::num::NonZeroU32;
use uuid::Uuid;

pub struct NotionBackup {
    credentials: NotionBackupCredentials,
    api_client: NotionApiClient,
    page_size: NonZeroU32,
}

pub struct NotionBackupParameters {
    pub credentials: NotionBackupCredentials,
    pub page_size: NonZeroU32,
}

pub struct NotionBackupBuilder {
    pub page_size: NonZeroU32,
}

fn api_client_error(error: ureq::Error) -> Error {
    Error::Backup(eyre::Error::new(error))
}

impl NotionBackup {
    pub fn new(parameters: NotionBackupParameters) -> Result<Self> {
        let NotionBackupParameters {
            credentials,
            page_size,
        } = parameters;

        let api_client = NotionApiClient::new(NotionApiClientParameters {
            api_key: credentials.api_key().to_string(),
            base_url_override: None,
        });

        Ok(Self {
            api_client,
            credentials,
            page_size,
        })
    }
}

impl NotionBackupBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> Result<NotionBackup> {
        let BackupCredentials::Notion(credentials) = credentials;

        NotionBackup::new(NotionBackupParameters {
            credentials,
            page_size: self.page_size,
        })
    }
}

impl BackupServiceBuilder<NotionBackup> for NotionBackupBuilder {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<NotionBackup> {
        self.build(credentials.clone())
    }
}

impl BackupService for NotionBackup {}

impl ListCommandsBackup for NotionBackup {
    fn list_commands_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Command>, Option<String>)>> {
        let database_id = self.credentials.commands_database_id();

        let response = self
            .api_client
            .query_database(QueryDatabaseParameters {
                database_id,
                start_cursor: page_id,
                page_size: Some(self.page_size),
                filter: None,
            })
            .map_err(api_client_error)?;

        let database_query_response: QueryDatabaseResponse<NotionCommandProperties> =
            query_datrabase_response(response)?;

        let next_page_token = database_query_response.next_cursor;

        let commands = database_query_response
            .database_pages
            .into_iter()
            .map(|page| {
                let id = page.properties.external_id;
                let workspace_id: Uuid = page.properties.workspace_id.parse().map_err(|_err| {
                    Error::InvalidArgument(format!("Invalid workspace ID: {}", id))
                })?;

                Command::new(CommandParameters {
                    id: id.parse().map_err(|_err| {
                        Error::InvalidArgument(format!("Invalid workspace ID: {}", id))
                    })?,
                    last_execute_time: None,
                    program: page.properties.program,
                    name: page.properties.name,
                    workspace_id: workspace_id.into(),
                })
            })
            .collect::<Result<Vec<Command>>>()?;

        Ok(Some((commands, next_page_token)))
    }
}

impl ListWorkspacesBackup for NotionBackup {
    fn list_workspaces_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Workspace>, Option<String>)>> {
        let database_id = self.credentials.workspaces_database_id();

        let response = self
            .api_client
            .query_database(QueryDatabaseParameters {
                database_id,
                start_cursor: page_id,
                page_size: Some(self.page_size),
                filter: None,
            })
            .map_err(api_client_error)?;

        let database_query_response: QueryDatabaseResponse<NotionWorkspaceProperties> =
            query_datrabase_response(response)?;

        let next_page_token = database_query_response.next_cursor;

        let workspaces = database_query_response
            .database_pages
            .into_iter()
            .map(|page| {
                let id = page.properties.external_id;

                Workspace::new(WorkspaceParameters {
                    id: id.parse().map_err(|_err| {
                        Error::InvalidArgument(format!("Invalid workspace ID: {}", id))
                    })?,
                    last_access_time: None,
                    location: Some(page.properties.location),
                    name: page.properties.name,
                })
            })
            .collect::<Result<Vec<Workspace>>>()?;

        Ok(Some((workspaces, next_page_token)))
    }
}

impl UpsertCommandsBackup for NotionBackup {
    fn upsert_commands_backup(&self, commands: Vec<Command>) -> Result<()> {
        if commands.is_empty() {
            return Ok(());
        }

        let database_id = self.credentials.commands_database_id();
        let external_ids: Vec<String> = commands
            .iter()
            .map(|command| command.id().to_string())
            .collect();
        let count = external_ids.len();

        let filter = external_ids_filter(external_ids);

        let response = self
            .api_client
            .query_database(QueryDatabaseParameters {
                database_id,
                start_cursor: None,
                page_size: NonZeroU32::new(count as u32),
                filter,
            })
            .map_err(api_client_error)?;

        let response: QueryDatabaseResponse<NotionCommandProperties> =
            query_datrabase_response(response)?;

        for command in commands {
            let page = response
                .database_pages
                .iter()
                .find(|p| p.properties.external_id == command.id().to_string());

            let Some(page) = page else {
                self.api_client.create_database_entry(CreateDatabaseEntryParameters {
                    database_id,
                    properties: serde_json::json!({
                        "Name": {"title": [{"text": {"content": command.name()}}]},
                        "External ID": {"rich_text": [{"text": {"content": command.id().to_string()}}]},
                        "Program": {"rich_text": [{"text": {"content": command.program()}}]},
                        "Workspace ID": {"rich_text": [{"text": {"content": command.workspace_id().to_string()}}]}
                    }),
                }).map_err(api_client_error)?;

                continue;
            };

            if command.name() != page.properties.name
                || command.program() != page.properties.program
            {
                self.api_client
                    .update_database_entry(UpdateDatabaseEntryParameters {
                        entry_id: &page.page_id,
                        properties: serde_json::json!({
                            "Name": {"title": [{"text": {"content": command.name()}}]},
                            "Program": {"rich_text": [{"text": {"content": command.program()}}]}
                        }),
                    })
                    .map_err(api_client_error)?;
            }
        }

        Ok(())
    }
}

impl UpsertWorkspacesBackup for NotionBackup {
    fn upsert_workspaces_backup(&self, workspaces: Vec<Workspace>) -> Result<()> {
        if workspaces.is_empty() {
            return Ok(());
        }

        let database_id = self.credentials.workspaces_database_id();
        let external_ids: Vec<String> = workspaces
            .iter()
            .map(|command| command.id().to_string())
            .collect();
        let count = external_ids.len();

        let filter = external_ids_filter(external_ids);

        let response = self
            .api_client
            .query_database(QueryDatabaseParameters {
                database_id,
                start_cursor: None,
                page_size: NonZeroU32::new(count as u32),
                filter,
            })
            .map_err(api_client_error)?;

        let response: QueryDatabaseResponse<NotionWorkspaceProperties> =
            query_datrabase_response(response)?;

        for workspace in workspaces {
            let page = response
                .database_pages
                .iter()
                .find(|p| p.properties.external_id == workspace.id().to_string());

            let Some(page) = page else {
                self.api_client.create_database_entry(CreateDatabaseEntryParameters {
                    database_id,
                    properties: serde_json::json!({
                        "Name": {"title": [{"text": {"content": workspace.name()}}]},
                        "External ID": {"rich_text": [{"text": {"content": workspace.id().to_string()}}]},
                        "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                    }),
                }).map_err(api_client_error)?;

                continue;
            };

            if workspace.name() != page.properties.name
                || workspace.location().unwrap_or_default() != page.properties.location
            {
                self.api_client
                    .update_database_entry(UpdateDatabaseEntryParameters {
                        entry_id: &page.page_id,
                        properties: serde_json::json!({
                            "Name": {"title": [{"text": {"content": workspace.name()}}]},
                            "Location": {"rich_text": [{"text": {"content": workspace.location()}}]}
                        }),
                    })
                    .map_err(api_client_error)?;
            }
        }

        Ok(())
    }
}

impl VerifyBackupCredentials for NotionBackup {
    fn verify_backup_credentials(&self) -> Result<bool> {
        let database_id = self.credentials.commands_database_id();
        let response = self
            .api_client
            .get_database_properties(database_id)
            .map_err(api_client_error)?;
        let properties = get_database_properties(response)?;

        if !verify_commands_database_properties(properties) {
            return Ok(false);
        }

        let database_id = self.credentials.workspaces_database_id();
        let response = self
            .api_client
            .get_database_properties(database_id)
            .map_err(api_client_error)?;
        let properties = get_database_properties(response)?;

        if !verify_workspaces_database_properties(properties) {
            return Ok(false);
        }

        Ok(true)
    }
}
