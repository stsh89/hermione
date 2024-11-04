use crate::{
    definitions::Workspace,
    services::{CreateWorkspace, NewWorkspaceParameters, StorageProvider},
    Result,
};

pub struct CreateWorkspaceOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
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
