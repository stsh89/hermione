use crate::{Route, WorkspaceCommandsRoute, WorkspacesRoute};
use std::num::NonZeroU32;
use uuid::Uuid;

pub const LIST_WORKSPACES_PAGE_SIZE: u32 = 100;
pub const LIST_WORKSPACE_COMMANDS_PAGE_SIZE: u32 = 100;

pub struct CreateWorkspaceParams {
    pub name: String,
    pub location: String,
}

pub struct CreateWorkspaceCommandParams {
    pub name: String,
    pub program: String,
    pub workspace_id: Uuid,
}

pub struct DeleteWorkspaceParams {
    pub id: Uuid,
}

pub struct DeleteCommandParams {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
}

pub struct EditWorkspaceParams {
    pub id: Uuid,
}

pub struct EditCommandParams {
    pub command_id: Uuid,
}

pub struct ListWorkspacesParams {
    pub search_query: String,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
}

pub struct ListWorkspaceCommandsParams {
    pub workspace_id: Uuid,
    pub search_query: String,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub powershell_no_exit: bool,
}

pub struct NewWorkspaceCommandParams {
    pub workspace_id: Uuid,
}

pub struct CopyCommandToClipboardParams {
    pub command_id: Uuid,
}

pub struct ExecuteCommandParams {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
    pub powershell_no_exit: bool,
}

pub struct OpenWindowsTerminalParams {
    pub working_directory: String,
}

pub struct UpdateWorkspaceParams {
    pub id: Uuid,
    pub name: String,
    pub location: String,
}

pub struct UpdateWorkspaceCommandParams {
    pub command_id: Uuid,
    pub name: String,
    pub program: String,
    pub workspace_id: Uuid,
}

impl Default for ListWorkspacesParams {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            page_number: NonZeroU32::new(1),
            page_size: NonZeroU32::new(LIST_WORKSPACES_PAGE_SIZE),
        }
    }
}

impl From<CreateWorkspaceParams> for Route {
    fn from(value: CreateWorkspaceParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Create(value))
    }
}

impl From<CreateWorkspaceCommandParams> for Route {
    fn from(value: CreateWorkspaceCommandParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Create(
            value,
        )))
    }
}

impl From<DeleteWorkspaceParams> for Route {
    fn from(value: DeleteWorkspaceParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Delete(value))
    }
}

impl From<DeleteCommandParams> for Route {
    fn from(parameters: DeleteCommandParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Delete(
            parameters,
        )))
    }
}

impl From<EditWorkspaceParams> for Route {
    fn from(value: EditWorkspaceParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Edit(value))
    }
}

impl From<EditCommandParams> for Route {
    fn from(parameters: EditCommandParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Edit(
            parameters,
        )))
    }
}

impl From<ListWorkspacesParams> for Route {
    fn from(value: ListWorkspacesParams) -> Self {
        Self::Workspaces(WorkspacesRoute::List(value))
    }
}

impl From<ListWorkspaceCommandsParams> for Route {
    fn from(value: ListWorkspaceCommandsParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::List(
            value,
        )))
    }
}

impl From<NewWorkspaceCommandParams> for Route {
    fn from(parameters: NewWorkspaceCommandParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::New(
            parameters,
        )))
    }
}

impl From<UpdateWorkspaceParams> for Route {
    fn from(value: UpdateWorkspaceParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Update(value))
    }
}

impl From<UpdateWorkspaceCommandParams> for Route {
    fn from(value: UpdateWorkspaceCommandParams) -> Self {
        Self::Workspaces(WorkspacesRoute::Commands(WorkspaceCommandsRoute::Update(
            value,
        )))
    }
}
