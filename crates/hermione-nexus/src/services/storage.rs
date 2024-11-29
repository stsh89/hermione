use crate::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, Workspace, WorkspaceId,
    },
    Result,
};

pub trait StorageService {}

pub trait CreateCommand: StorageService {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command>;
}

pub trait CreateWorkspace: StorageService {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace>;
}

pub trait DeleteBackupCredentials: StorageService {
    fn delete_backup_credentials(&self, kind: BackupProviderKind) -> Result<()>;
}

pub trait DeleteCommand: StorageService {
    fn delete_command(&self, id: CommandId) -> Result<()>;
}

pub trait DeleteWorkspaceCommands: StorageService {
    fn delete_workspace_commands(&self, id: WorkspaceId) -> Result<()>;
}

pub trait DeleteWorkspace: StorageService {
    fn delete_workspace(&self, id: WorkspaceId) -> Result<()>;
}

pub trait FindBackupCredentials: StorageService {
    fn find_backup_credentials(
        &self,
        kind: BackupProviderKind,
    ) -> Result<Option<BackupCredentials>>;
}

pub trait FindCommand: StorageService {
    fn find_command(&self, id: CommandId) -> Result<Option<Command>>;
}

pub trait FindWorkspace: StorageService {
    fn find_workspace(&self, id: WorkspaceId) -> Result<Option<Workspace>>;
}

pub trait ListBackupCredentials: StorageService {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>>;
}

pub trait ListCommands: StorageService {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListWorkspaces: StorageService {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait SaveBackupCredentials: StorageService {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<()>;
}

pub trait TrackCommandExecuteTime: StorageService {
    fn track_command_execute_time(&self, command_id: CommandId) -> Result<()>;
}

pub trait TrackWorkspaceAccessTime: StorageService {
    fn track_workspace_access_time(&self, workspace_id: WorkspaceId) -> Result<()>;
}

pub trait UpdateCommand: StorageService {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()>;
}

pub trait UpdateWorkspace: StorageService {
    fn update_workspace(&self, workspace: EditWorkspaceParameters) -> Result<()>;
}

pub trait UpsertCommands: StorageService {
    fn upsert_commands(&self, commands: Vec<Command>) -> Result<()>;
}

pub trait UpsertWorkspaces: StorageService {
    fn upsert_workspaces(&self, workspaces: Vec<Workspace>) -> Result<()>;
}

pub struct EditCommandParameters<'a> {
    pub id: CommandId,
    pub name: &'a str,
    pub program: &'a str,
}

pub struct EditWorkspaceParameters<'a> {
    pub id: WorkspaceId,
    pub name: &'a str,
    pub location: Option<&'a str>,
}

pub struct FilterCommandsParameters<'a> {
    pub program_contains: Option<&'a str>,
    pub page_number: u32,
    pub page_size: u32,
    pub workspace_id: Option<WorkspaceId>,
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
