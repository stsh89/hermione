use super::GetBackupCredentialsOperation;
use crate::{
    definitions::{BackupCredentials, BackupProviderKind},
    services::{DeleteBackupCredentials, FindBackupCredentials, StorageService},
    Result,
};

pub struct DeleteBackupCredentialsOperation<'a, DBC, FBC>
where
    DBC: StorageService,
    FBC: StorageService,
{
    pub delete_provider: &'a DBC,
    pub find_provider: &'a FBC,
}

impl<'a, DBC, FBC> DeleteBackupCredentialsOperation<'a, DBC, FBC>
where
    DBC: DeleteBackupCredentials,
    FBC: FindBackupCredentials,
{
    pub fn execute(&self, kind: BackupProviderKind) -> Result<()> {
        tracing::info!(operation = "Delete backup credentials");

        self.get_backup_credentials(kind)?;
        self.delete_provider.delete_backup_credentials(kind)
    }

    fn get_backup_credentials(&self, kind: BackupProviderKind) -> Result<BackupCredentials> {
        GetBackupCredentialsOperation {
            provider: self.find_provider,
        }
        .execute(kind)
    }
}
