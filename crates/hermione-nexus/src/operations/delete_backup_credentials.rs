use super::GetBackupCredentialsOperation;
use crate::{
    definitions::BackupProviderKind,
    services::{DeleteBackupCredentials, FindBackupCredentials},
    Result,
};

pub struct DeleteBackupCredentialsOperation<'a, D, F> {
    pub find_provider: &'a F,
    pub delete_provider: &'a D,
}

impl<'a, D, F> DeleteBackupCredentialsOperation<'a, D, F>
where
    D: DeleteBackupCredentials,
    F: FindBackupCredentials,
{
    pub fn execute(&self, kind: &BackupProviderKind) -> Result<()> {
        GetBackupCredentialsOperation {
            provider: self.find_provider,
        }
        .execute(kind)?;

        self.delete_provider.delete_backup_credentials(kind)
    }
}
