use super::Result;
use crate::definitions::{BackupCredentials, Command, Workspace};

pub trait BackupService {}

pub trait BackupServiceBuilder<T> {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<T>;
}

pub trait ListCommandsBackup: BackupService {
    fn list_commands_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Command>, Option<String>)>>;
}

pub trait ListWorkspacesBackup: BackupService {
    fn list_workspaces_backup(
        &self,
        page_id: Option<&str>,
    ) -> Result<Option<(Vec<Workspace>, Option<String>)>>;
}

pub trait BackupCommand: BackupService {
    fn backup_command(&self, command: Command) -> Result<()>;
}

pub trait BackupCommands: BackupService {
    fn backup_commands(&self, commands: Vec<Command>) -> Result<()>;
}

pub trait BackupWorkspaces: BackupService {
    fn backup_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()>;
}

pub trait BackupWorkspace: BackupService {
    fn backup_workspace(&self, workspace: Workspace) -> Result<()>;
}

pub trait VerifyBackupCredentials: BackupService {
    fn verify_backup_credentials(&self) -> Result<()>;
}
