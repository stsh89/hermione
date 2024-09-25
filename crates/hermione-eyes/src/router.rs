#[derive(Clone)]
pub enum Router {
    /// Get workspace
    GetWorkspace(GetWorkspaceParameters),

    /// List workspaces filtered by search query
    ListWorkspaces(ListWorkspacesParameters),

    /// New workspace
    NewWorkspace,

    /// Delete workspace
    DeleteWorkspace,

    /// Delete command
    DeleteCommand,

    /// Get command
    GetCommand(GetCommandParameters),

    /// New command
    NewCommand,

    /// Create workspace
    CreateWorkspace(CreateWorkspaceParameters),

    /// Create command
    CreateCommand(CreateCommandParameters),
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
