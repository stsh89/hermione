#[derive(Clone, PartialEq)]
pub enum Router {
    /// List workspaces filtered by search query
    ListWorkspaces(ListWorkspacesParameters),

    /// New workspace
    NewWorkspace,

    /// Create workspace
    CreateWorkspace(CreateWorkspaceParameters),
}

#[derive(Clone, PartialEq)]
pub struct CreateWorkspaceParameters {
    pub name: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct ListWorkspacesParameters {
    pub search_query: String,
}
