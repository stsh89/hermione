use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::FindBackupCredentials,
    Error, Result,
};

pub struct GetBackupCredentials<'a, F> {
    pub provider: &'a F,
}

impl<'a, F> GetBackupCredentials<'a, F>
where
    F: FindBackupCredentials,
{
    pub fn execute(&self, backup_provider_kind: &BackupProviderKind) -> Result<BackupCredentials> {
        tracing::info!(operation = "Get backup credentials");

        let credentials = self
            .provider
            .find_backup_credentials(backup_provider_kind)?
            .ok_or(Error::NotFound("Backup credentials".to_string()))?;

        Ok(credentials)
    }
}
