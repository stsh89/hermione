use crate::{
    services::storage::{CreateWorkspace, NewWorkspaceParameters, Workspace},
    Result,
};

pub struct CreateWorkspaceOperation<'a, CW> {
    pub operator: &'a CW,
}

pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: Option<String>,
}

impl<'a, CW> CreateWorkspaceOperation<'a, CW>
where
    CW: CreateWorkspace,
{
    pub fn execute(&self, parameters: CreateWorkspaceParameters) -> Result<Workspace> {
        tracing::info!(operation = "Create workspace");

        let CreateWorkspaceParameters { name, location } = parameters;

        self.operator
            .create_workspace(NewWorkspaceParameters { name, location })
    }
}
