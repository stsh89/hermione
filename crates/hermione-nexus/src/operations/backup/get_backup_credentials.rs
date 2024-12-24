use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{FindBackupCredentials, StorageService},
    Error, Result,
};
use eyre::eyre;

pub struct GetBackupCredentialsOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

impl<F> GetBackupCredentialsOperation<'_, F>
where
    F: FindBackupCredentials,
{
    pub fn execute(&self, kind: BackupProviderKind) -> Result<BackupCredentials> {
        tracing::info!(operation = "Get backup credentials");

        self.provider
            .find_backup_credentials(kind)?
            .ok_or(eyre!("Could not find {} backup credentials", kind))
            .map_err(Error::not_found)
    }
}
