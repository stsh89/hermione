use crate::{
    CreateWorkspaceCommandParameters, CreateWorkspaceParameters, DeleteWorkspaceCommandParameters,
    DeleteWorkspaceParameters, EditWorkspaceCommandParameters, EditWorkspaceParameters,
    ListWorkspaceCommandsParameters, ListWorkspacesParameters, NewWorkspaceCommandParameters,
    PowerShellCopyToClipboardParameters, PowerShellExecuteCommandParameters,
    PowerShellOpenWindowsTerminalParameters, UpdateWorkspaceCommandParameters,
    UpdateWorkspaceParameters,
};

pub enum Route {
    Powershell(PowerShellRoute),
    Workspaces(WorkspacesRoute),
}

pub enum PowerShellRoute {
    ExecuteCommand(PowerShellExecuteCommandParameters),
    CopyToClipboard(PowerShellCopyToClipboardParameters),
    OpenWindowsTerminal(PowerShellOpenWindowsTerminalParameters),
}

pub enum WorkspacesRoute {
    Commands(WorkspaceCommandsRoute),
    Create(CreateWorkspaceParameters),
    Delete(DeleteWorkspaceParameters),
    Edit(EditWorkspaceParameters),
    List(ListWorkspacesParameters),
    New,
    Update(UpdateWorkspaceParameters),
}

pub enum WorkspaceCommandsRoute {
    Create(CreateWorkspaceCommandParameters),
    Delete(DeleteWorkspaceCommandParameters),
    Edit(EditWorkspaceCommandParameters),
    List(ListWorkspaceCommandsParameters),
    New(NewWorkspaceCommandParameters),
    Update(UpdateWorkspaceCommandParameters),
}
