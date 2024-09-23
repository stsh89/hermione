pub enum Router {
    ListWorkspaces,
    NewWorkspace,
    CreateWorkspace(CreateWorkspaceParameters),
}

pub struct CreateWorkspaceParameters {
    pub name: String,
}
