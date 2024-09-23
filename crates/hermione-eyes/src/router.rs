#[derive(Clone)]
pub enum Router {
    ListWorkspaces,
    NewWorkspace,
    CreateWorkspace(CreateWorkspaceParameters),
}

#[derive(Clone)]
pub struct CreateWorkspaceParameters {
    pub name: String,
}
