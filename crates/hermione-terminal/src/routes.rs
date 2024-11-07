use crate::{
    CopyCommandToClipboardParams, CreateWorkspaceCommandParams, CreateWorkspaceParams,
    DeleteBackupCredentialsParams, DeleteCommandParams, DeleteWorkspaceParams, EditCommandParams,
    EditWorkspaceParams, ExecuteCommandParams, ImportParams, ListWorkspaceCommandsParams,
    ListWorkspacesParams, NewWorkspaceCommandParams, OpenWindowsTerminalParams,
    SaveNotionBackupCredentialsParams, UpdateWorkspaceCommandParams, UpdateWorkspaceParams,
};

pub enum Route {
    BackupCredentials(BackupCredentialsRoute),
    Powershell(PowerShellRoute),
    Workspaces(WorkspacesRoute),
}

pub enum BackupCredentialsRoute {
    List,
    ManageNotionBackupCredentials,
    SaveNotionBackupCredentials(SaveNotionBackupCredentialsParams),
    DeleteBackupCredentials(DeleteBackupCredentialsParams),
    Import(ImportParams),
}

pub enum PowerShellRoute {
    ExecuteCommand(ExecuteCommandParams),
    CopyToClipboard(CopyCommandToClipboardParams),
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
    Delete(DeleteCommandParams),
    Edit(EditCommandParams),
    List(ListWorkspaceCommandsParams),
    New(NewWorkspaceCommandParams),
    Update(UpdateWorkspaceCommandParams),
}
