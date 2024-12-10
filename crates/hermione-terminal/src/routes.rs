use crate::{
    CopyCommandToClipboardParams, CreateWorkspaceCommandParams, CreateWorkspaceParams,
    DeleteBackupCredentialsParams, DeleteCommandParams, DeleteWorkspaceParams, EditCommandParams,
    EditWorkspaceParams, ExecuteCommandParams, ExecuteProgramParams, ExportParams, ImportParams,
    ListWorkspaceCommandsParams, ListWorkspacesParams, NewWorkspaceCommandParams,
    OpenWindowsTerminalParams, SaveNotionBackupCredentialsParams, UpdateWorkspaceCommandParams,
    UpdateWorkspaceParams,
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
    Export(ExportParams),
}

pub enum PowerShellRoute {
    ExecuteCommand(ExecuteCommandParams),
    ExecuteProgram(ExecuteProgramParams),
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

impl From<WorkspacesRoute> for Route {
    fn from(route: WorkspacesRoute) -> Self {
        Route::Workspaces(route)
    }
}
