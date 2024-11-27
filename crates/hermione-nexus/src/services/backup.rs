use crate::{
    definitions::{BackupCredentials, Command, Workspace},
    Result,
};

pub struct BackupCopyParameters<'a> {
    pub page_token: Option<&'a str>,
}

pub struct BackupCopies<T> {
    pub copies: Vec<T>,
    pub next_page_token: Option<String>,
}

pub trait BackupService {}

pub trait BackupServiceBuilder<T> {
    fn build_backup_provider(&self, credentials: &BackupCredentials) -> Result<T>;
}

pub trait GetCommandsBackupCopy: BackupService {
    fn get_commands_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Command>>;
}

pub trait GetWorkspacesBackupCopy: BackupService {
    fn get_workspaces_backup_copy(
        &self,
        parameters: BackupCopyParameters,
    ) -> Result<BackupCopies<Workspace>>;
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
