use crate::{
    CopyToClipboardParams, CreateWorkspaceCommandParams, CreateWorkspaceParams,
    DeleteWorkspaceCommandParams, DeleteWorkspaceParams, EditWorkspaceCommandParams,
    EditWorkspaceParams, ExecuteCommandParams, ListWorkspaceCommandsParams, ListWorkspacesParams,
    NewWorkspaceCommandParams, OpenWindowsTerminalParams, UpdateWorkspaceCommandParams,
    UpdateWorkspaceParams,
};

pub enum Route {
    Powershell(PowerShellRoute),
    Workspaces(WorkspacesRoute),
}

pub enum PowerShellRoute {
    ExecuteCommand(ExecuteCommandParams),
    CopyToClipboard(CopyToClipboardParams),
    OpenWindowsTerminal(OpenWindowsTerminalParams),
}

pub enum WorkspacesRoute {
    Commands(WorkspaceCommandsRoute),
    Create(CreateWorkspaceParams),
    Delete(DeleteWorkspaceParams),
    Edit(EditWorkspaceParams),
    List(ListWorkspacesParams),
    New,
    Update(UpdateWorkspaceParams),
}

pub enum WorkspaceCommandsRoute {
    Create(CreateWorkspaceCommandParams),
    Delete(DeleteWorkspaceCommandParams),
    Edit(EditWorkspaceCommandParams),
    List(ListWorkspaceCommandsParams),
    New(NewWorkspaceCommandParams),
    Update(UpdateWorkspaceCommandParams),
}
