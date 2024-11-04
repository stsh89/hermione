use super::Result;
use crate::definitions::{BackupCredentials, Command, Workspace};

pub trait BackupProvider {}

pub trait ListCommandsBackup: BackupProvider {
    fn list_commands_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Command>, Option<String>)>>;
}

pub trait ListWorkspacesBackup: BackupProvider {
    fn list_workspaces_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Workspace>, Option<String>)>>;
}

pub trait BackupProviderBuilder<T> {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<T>;
}

pub trait VerifyBackupCredentials: BackupProvider {
    fn verify_backup_credentials(&self) -> Result<bool>;
}