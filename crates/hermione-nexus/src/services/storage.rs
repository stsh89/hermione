use crate::{
    definitions::{Command, CommandId, Workspace, WorkspaceId},
    Result,
};

pub trait StorageProvider {}

pub trait CreateCommand: StorageProvider {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command>;
}

pub trait CreateWorkspace: StorageProvider {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace>;
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

pub trait FindCommand: StorageProvider {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>>;
}

pub trait FindWorkspace: StorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>>;
}

pub trait ListCommands: StorageProvider {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListWorkspaces: StorageProvider {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait UpdateCommand: StorageProvider {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<Command>;
}

pub trait UpdateWorkspace: StorageProvider {
    fn update_workspace(&self, workspace: EditWorkspaceParameters) -> Result<Workspace>;
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
