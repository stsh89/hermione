use crate::{
    definitions::Workspace,
    services::{CreateWorkspace, NewWorkspaceParameters, StorageService},
    Result,
};

pub struct CreateWorkspaceOperation<'a, SP>
where
    SP: StorageService,
{
    pub storage_provider: &'a SP,
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

        self.storage_provider
            .create_workspace(NewWorkspaceParameters { name, location })
    }
}
