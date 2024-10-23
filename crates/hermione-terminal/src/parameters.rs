use crate::{Route, WorkspaceCommandsRoute, WorkspacesRoute};

pub const LIST_WORKSPACES_PAGE_SIZE: u32 = 100;
pub const LIST_WORKSPACE_COMMANDS_PAGE_SIZE: u32 = 100;

pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: String,
}

pub struct CreateWorkspaceCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct DeleteWorkspaceParameters {
    pub id: String,
}

pub struct DeleteWorkspaceCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct EditWorkspaceParameters {
    pub id: String,
}

pub struct EditWorkspaceCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct ListWorkspacesParameters {
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListWorkspaceCommandsParameters {
    pub workspace_id: String,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
    pub powershell_no_exit: bool,
}

pub struct NewWorkspaceCommandParameters {
    pub workspace_id: String,
}

pub struct CopyToClipboardParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct PowerShellExecuteCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
    pub powershell_no_exit: bool,
}

pub struct OpenWindowsTerminalParameters {
    pub working_directory: String,
}

pub struct UpdateWorkspaceParameters {
    pub id: String,
    pub name: String,
    pub location: String,
}

pub struct UpdateWorkspaceCommandParameters {
    pub command_id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

impl Default for ListWorkspacesParameters {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            page_number: 0,
            page_size: LIST_WORKSPACES_PAGE_SIZE,
        }
    }
}

impl From<CreateWorkspaceParameters> for Route {
    fn from(value: CreateWorkspaceParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Create(value))
    }
}

impl From<CreateWorkspaceCommandParameters> for Route {
    fn from(value: CreateWorkspaceCommandParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Create(
            value,
        )))
    }
}

impl From<DeleteWorkspaceParameters> for Route {
    fn from(value: DeleteWorkspaceParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Delete(value))
    }
}

impl From<DeleteWorkspaceCommandParameters> for Route {
    fn from(parameters: DeleteWorkspaceCommandParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Delete(
            parameters,
        )))
    }
}

impl From<EditWorkspaceParameters> for Route {
    fn from(value: EditWorkspaceParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Edit(value))
    }
}

impl From<EditWorkspaceCommandParameters> for Route {
    fn from(parameters: EditWorkspaceCommandParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Edit(
            parameters,
        )))
    }
}

impl From<ListWorkspacesParameters> for Route {
    fn from(value: ListWorkspacesParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::List(value))
    }
}

impl From<ListWorkspaceCommandsParameters> for Route {
    fn from(value: ListWorkspaceCommandsParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::List(
            value,
        )))
    }
}

impl From<NewWorkspaceCommandParameters> for Route {
    fn from(parameters: NewWorkspaceCommandParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::New(
            parameters,
        )))
    }
}

impl From<UpdateWorkspaceParameters> for Route {
    fn from(value: UpdateWorkspaceParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Update(value))
    }
}

impl From<UpdateWorkspaceCommandParameters> for Route {
    fn from(value: UpdateWorkspaceCommandParameters) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Update(
            value,
        )))
    }
}
