use crate::{
    definitions::BackupProviderKind,
    services::{DeleteBackupCredentials, FindBackupCredentials, StorageProvider},
    Error, Result,
};

pub struct DeleteBackupCredentialsOperation<'a, DBC, FBC>
where
    DBC: StorageProvider,
    FBC: StorageProvider,
{
    pub delete_provider: &'a DBC,
    pub find_provider: &'a FBC,
}

impl<'a, DBC, FBC> DeleteBackupCredentialsOperation<'a, DBC, FBC>
where
    DBC: DeleteBackupCredentials,
    FBC: FindBackupCredentials,
{
    pub fn execute(&self, kind: &BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Delete backup credentials");

        if self.find_provider.find_backup_credentials(kind)?.is_none() {
            return Err(Error::NotFound("Backup credentials".to_string()));
        }

        self.delete_provider.delete_backup_credentials(kind)
    }
}
