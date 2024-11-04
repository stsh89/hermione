use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{FindBackupCredentials, StorageProvider},
    Error, Result,
};

pub struct GetBackupCredentialsOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

impl<'a, F> GetBackupCredentialsOperation<'a, F>
where
    F: FindBackupCredentials,
{
    pub fn execute(&self, kind: &BackupProviderKind) -> Result<BackupCredentials> {
        tracing::info!(operation = "Get backup credentials");

        self.provider
            .find_backup_credentials(kind)?
            .ok_or(Error::NotFound("Backup credentials".to_string()))
    }
}
