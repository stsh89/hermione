use crate::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, Workspace, WorkspaceId,
    },
    Result,
};

pub trait StorageProvider {}

pub trait CreateCommand: StorageProvider {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command>;
}

pub trait CreateWorkspace: StorageProvider {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace>;
}

pub trait DeleteBackupCredentials: StorageProvider {
    fn delete_backup_credentials(&self, kind: &BackupProviderKind) -> Result<()>;
}

pub trait DeleteCommand: StorageProvider {
    fn delete_command(&self, id: &CommandId) -> Result<()>;
}

pub trait DeleteWorkspaceCommands: StorageProvider {
    fn delete_workspace_commands(&self, id: &WorkspaceId) -> Result<()>;
}

pub trait DeleteWorkspace: StorageProvider {
    fn delete_workspace(&self, id: &WorkspaceId) -> Result<()>;
}

pub trait FindBackupCredentials: StorageProvider {
    fn find_backup_credentials(
        &self,
        kind: &BackupProviderKind,
    ) -> Result<Option<BackupCredentials>>;
}

pub trait FindCommand: StorageProvider {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>>;
}

pub trait FindWorkspace: StorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>>;
}

pub trait ListBackupCredentials: StorageProvider {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>>;
}

pub trait ListCommands: StorageProvider {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListWorkspaces: StorageProvider {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait SaveBackupCredentials: StorageProvider {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<()>;
}

pub trait TrackCommandExecuteTime: StorageProvider {
    fn track_command_execute_time(&self, command_id: &CommandId) -> Result<()>;
}

pub trait TrackWorkspaceAccessTime: StorageProvider {
    fn track_workspace_access_time(&self, workspace_id: &WorkspaceId) -> Result<()>;
}

pub trait UpdateCommand: StorageProvider {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()>;
}

pub trait UpdateWorkspace: StorageProvider {
    fn update_workspace(&self, workspace: EditWorkspaceParameters) -> Result<()>;
}

pub trait UpsertCommands: StorageProvider {
    fn upsert_commands(&self, commands: Vec<Command>) -> Result<()>;
}

pub trait UpsertWorkspaces: StorageProvider {
    fn upsert_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()>;
}

pub struct EditCommandParameters<'a> {
    pub id: &'a CommandId,
    pub name: &'a str,
    pub program: &'a str,
}

pub struct EditWorkspaceParameters<'a> {
    pub id: &'a WorkspaceId,
    pub name: &'a str,
    pub location: Option<&'a str>,
}

pub struct FilterCommandsParameters<'a> {
    pub program_contains: Option<&'a str>,
    pub page_number: u32,
    pub page_size: u32,
    pub workspace_id: Option<&'a WorkspaceId>,
}

pub struct FilterWorkspacesParameters<'a> {
    pub name_contains: Option<&'a str>,
    pub page_number: u32,
    pub page_size: u32,
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}

pub struct NewWorkspaceParameters {
    pub name: String,
    pub location: Option<String>,
}
