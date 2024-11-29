use super::GetWorkspaceOperation;
use crate::{
    definitions::{Workspace, WorkspaceId},
    services::{DeleteWorkspace, FindWorkspace, StorageService},
    Result,
};

pub struct DeleteWorkspaceOperation<'a, FWP, DWP>
where
    FWP: StorageService,
    DWP: StorageService,
{
    pub find_workspace_provider: &'a FWP,
    pub delete_workspace_provider: &'a DWP,
}

impl<'a, FWP, DWP> DeleteWorkspaceOperation<'a, FWP, DWP>
where
    FWP: FindWorkspace,
    DWP: DeleteWorkspace,
{
    pub fn execute(&self, id: WorkspaceId) -> Result<()> {
        tracing::info!(operation = "Delete workspace");

        self.get_workspace(id)?;
        self.delete_workspace_provider.delete_workspace(id)?;

        Ok(())
    }

    fn get_workspace(&self, id: WorkspaceId) -> Result<Workspace> {
        GetWorkspaceOperation {
            provider: self.find_workspace_provider,
        }
        .execute(id)
    }
}
