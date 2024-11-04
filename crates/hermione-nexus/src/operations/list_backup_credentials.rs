use crate::{definitions::BackupCredentials, services::ListBackupCredentials, Result};

pub struct ListBackupCredentialsOperation<'a, L> {
    pub provider: &'a L,
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
