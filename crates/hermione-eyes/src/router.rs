#[derive(Clone)]
pub enum Router {
    /// List workspaces filtered by search query
    ListWorkspaces(ListWorkspacesParameters),

    /// New workspace
    NewWorkspace,

    /// Create workspace
    CreateWorkspace(CreateWorkspaceParameters),

    /// Command palette
    CommandPalette(CommandPaletteParameters),
}

#[derive(Clone)]
pub struct CreateWorkspaceParameters {
    pub name: String,
}

#[derive(Clone, Default)]
pub struct ListWorkspacesParameters {
    pub search_query: String,
}

#[derive(Clone)]
pub struct CommandPaletteParameters {
    pub commands: Vec<String>,
}
