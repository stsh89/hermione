use crate::{
    definitions::BackupCredentials,
    services::{ListBackupCredentials, StorageProvider},
    Result,
};

pub struct ListBackupCredentialsOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

impl<'a, L> ListBackupCredentialsOperation<'a, L>
where
    L: ListBackupCredentials,
{
    pub fn execute(&self) -> Result<Vec<BackupCredentials>> {
        tracing::info!(operation = "List backup credentials");

        self.provider.list_backup_credentials()
    }
}
