use crate::{
    definitions::Workspace,
    services::{CreateWorkspace, NewWorkspaceParameters},
    Result,
};

pub struct CreateWorkspaceOperation<'a, CW> {
    pub provider: &'a CW,
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

        self.provider
            .create_workspace(NewWorkspaceParameters { name, location })
    }
}
