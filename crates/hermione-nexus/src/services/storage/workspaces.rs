use crate::{
    definitions::{Workspace, WorkspaceId},
    Result, StorageProvider,
};

pub trait CreateWorkspace: StorageProvider {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace>;
}

pub trait FindWorkspace: StorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>>;
}

pub trait ListWorkspaces: StorageProvider {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait UpdateWorkspace: StorageProvider {
    fn update_workspace(&self, workspace: EditWorkspaceParameters) -> Result<Workspace>;
}

pub struct NewWorkspaceParameters {
    pub name: String,
    pub location: Option<String>,
}

pub struct EditWorkspaceParameters<'a> {
    pub id: &'a WorkspaceId,
    pub name: &'a str,
    pub location: Option<&'a str>,
}

pub struct FilterWorkspacesParameters<'a> {
    pub name_contains: Option<&'a str>,
    pub page_number: u32,
    pub page_size: u32,
}
