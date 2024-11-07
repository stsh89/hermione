use hermione_drive::notion::{
    get_database_properties, verify_commands_database_properties,
    verify_workspaces_database_properties, NotionApiClient, NotionApiClientParameters,
};
use hermione_nexus::{
    definitions::{BackupCredentials, NotionBackupCredentials},
    services::{BackupService, BackupServiceBuilder, VerifyBackupCredentials},
    Error, Result,
};

pub struct NotionBackup {
    credentials: NotionBackupCredentials,
    api_client: NotionApiClient,
}

#[derive(Default)]
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
