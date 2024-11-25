use crate::{
    definitions::{Workspace, WorkspaceId},
    services::{FindWorkspace, StorageService},
    Error, Result,
};

pub struct GetWorkspaceOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

impl<'a, F> GetWorkspaceOperation<'a, F>
where
    F: FindWorkspace,
{
    pub fn execute(&self, id: &WorkspaceId) -> Result<Workspace> {
        tracing::info!(operation = "Get workspace");

        self.provider
            .find_workspace(id)?
            .ok_or(Error::WorkspaceNotFound(**id))
    }
}
