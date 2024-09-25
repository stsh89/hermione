#[derive(Clone)]
pub enum Router {
    CreateCommand(CreateCommandParameters),
    CreateWorkspace(CreateWorkspaceParameters),
    DeleteCommand,
    DeleteWorkspace,
    EditWorkspace,
    ExecuteCommand(ExecuteCommandParameters),
    GetCommand(GetCommandParameters),
    GetWorkspace(GetWorkspaceParameters),
    ListWorkspaces(ListWorkspacesParameters),
    NewCommand,
    NewWorkspace,
    UpdateWorkspace(UpdateWorkspaceParameters),
}

#[derive(Clone)]
pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: String,
}

#[derive(Clone)]
pub struct CreateCommandParameters {
    pub name: String,
    pub program: String,
}

#[derive(Clone)]
pub struct ExecuteCommandParameters {
    pub number: usize,
}

#[derive(Clone, Default)]
pub struct ListWorkspacesParameters {
    pub search_query: String,
}

#[derive(Clone)]
pub struct GetWorkspaceParameters {
    pub number: usize,
    pub commands_search_query: String,
}

#[derive(Clone)]
pub struct GetCommandParameters {
    pub number: usize,
}

#[derive(Clone)]
pub struct UpdateWorkspaceParameters {
    pub name: String,
    pub location: String,
}
