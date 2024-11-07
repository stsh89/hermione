use std::num::NonZeroU32;

use hermione_drive::notion::{
    get_database_properties, query_datrabase_response, verify_commands_database_properties,
    verify_workspaces_database_properties, NotionApiClient, NotionApiClientParameters,
    NotionCommandProperties, NotionWorkspaceProperties, QueryDatabaseParameters,
    QueryDatabaseResponse,
};
use hermione_nexus::{
    definitions::{
        BackupCredentials, Command, CommandParameters, NotionBackupCredentials, Workspace,
        WorkspaceParameters,
    },
    services::{
        BackupService, BackupServiceBuilder, ListCommandsBackup, ListWorkspacesBackup,
        VerifyBackupCredentials,
    },
    Error, Result,
};
use uuid::Uuid;

pub struct NotionBackup {
    credentials: NotionBackupCredentials,
    api_client: NotionApiClient,
}

pub struct NotionBackupBuilder;

fn api_client_error(error: ureq::Error) -> Error {
    Error::Backup(eyre::Error::new(error))
}

impl NotionBackup {
    pub fn new(credentials: NotionBackupCredentials) -> Result<Self> {
        let api_client = NotionApiClient::new(NotionApiClientParameters {
            api_key: credentials.api_key().to_string(),
            base_url_override: None,
        });

        Ok(Self {
            api_client,
            credentials,
        })
    }
}

impl NotionBackupBuilder {
    pub fn build(&self, credentials: BackupCredentials) -> Result<NotionBackup> {
        let BackupCredentials::Notion(credentials) = credentials;

        NotionBackup::new(credentials)
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
                page_size: NonZeroU32::new(1),
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
                page_size: NonZeroU32::new(1),
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
