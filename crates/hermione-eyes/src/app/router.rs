#[derive(Clone)]
pub enum Router {
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

#[derive(Clone)]
pub struct CreateCommandParameters {
    pub workspace_id: String,
    pub name: String,
    pub program: String,
}

#[derive(Clone)]
pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: String,
}

#[derive(Clone)]
pub struct DeleteCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

#[derive(Clone)]
pub struct DeleteWorkspaceParameters {
    pub id: String,
}

#[derive(Clone)]
pub struct EditCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

#[derive(Clone)]
pub struct EditWorkspaceParameters {
    pub id: String,
}

#[derive(Clone)]
pub struct ExecuteCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
    pub execute_immediately: bool,
}

#[derive(Clone, Default)]
pub struct ListWorkspacesParameters {
    pub search_query: String,
}

#[derive(Clone)]
pub struct GetWorkspaceParameters {
    pub id: String,
    pub commands_search_query: String,
}

#[derive(Clone)]
pub struct GetCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
}

#[derive(Clone)]
pub struct NewCommandParameters {
    pub workspace_id: String,
}

#[derive(Clone)]
pub struct UpdateWorkspaceParameters {
    pub id: String,
    pub name: String,
    pub location: String,
}

#[derive(Clone)]
pub struct UpdateCommandParameters {
    pub workspace_id: String,
    pub command_id: String,
    pub name: String,
    pub program: String,
}
