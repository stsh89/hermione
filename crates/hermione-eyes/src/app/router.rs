pub enum Router {
    CopyToClipboard(CopyToClipboardParameters),
    CreateCommand(CreateCommandParameters),
    CreateWorkspace(CreateWorkspaceParameters),
    DeleteCommand(DeleteCommandParameters),
    DeleteWorkspace(DeleteWorkspaceParameters),
    EditWorkspace(EditWorkspaceParameters),
    EditCommand(EditCommandParameters),
    ExecuteCommand(ExecuteCommandParameters),
    GetCommand(GetCommandParameters),
    GetWorkspace(GetWorkspaceParameters),
    ListWorkspaces(ListWorkspacesParameters),
    NewCommand(NewCommandParameters),
    NewWorkspace,
    UpdateWorkspace(UpdateWorkspaceParameters),
    UpdateCommand(UpdateCommandParameters),
}

pub struct CopyToClipboardParameters {
    pub workspace_id: String,
    pub command_id: String,
}

pub struct CreateCommandParameters {
    pub workspace_id: String,
    pub name: String,
    pub program: String,
}

pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: String,
}

pub struct DeleteCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

pub struct DeleteWorkspaceParameters {
    pub id: String,
}

pub struct EditCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

pub struct EditWorkspaceParameters {
    pub id: String,
}

pub struct ExecuteCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
}

#[derive(Default)]
pub struct ListWorkspacesParameters {
    pub search_query: String,
}

pub struct GetWorkspaceParameters {
    pub id: String,
    pub commands_search_query: String,
}

pub struct GetCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

pub struct NewCommandParameters {
    pub workspace_id: String,
}

pub struct UpdateWorkspaceParameters {
    pub id: String,
    pub name: String,
    pub location: String,
}

pub struct UpdateCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
    pub name: String,
    pub program: String,
}
