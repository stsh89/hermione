use crate::{
    definitions::{Workspace, WorkspaceId},
    services::{FindWorkspace, StorageService},
    Error, Result,
};
use eyre::eyre;

pub struct GetWorkspaceOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

impl<F> GetWorkspaceOperation<'_, F>
where
    F: FindWorkspace,
{
    pub fn execute(&self, id: WorkspaceId) -> Result<Workspace> {
        tracing::info!(operation = "Get workspace");

        self.provider
            .find_workspace(id)?
            .ok_or(eyre!("Could not find workspace with ID: {}", id))
            .map_err(Error::not_found)
    }
}
