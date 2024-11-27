use super::GetWorkspaceOperation;
use crate::{
    definitions::{Workspace, WorkspaceId},
    services::{DeleteWorkspace, DeleteWorkspaceCommands, FindWorkspace, StorageService},
    Result,
};

pub struct DeleteWorkspaceOperation<'a, FWP, DWCP, DWP>
where
    FWP: StorageService,
    DWCP: StorageService,
    DWP: StorageService,
{
    pub find_workspace_provider: &'a FWP,
    pub delete_workspace_commands_provider: &'a DWCP,
    pub delete_workspace_provider: &'a DWP,
}

impl<'a, FWP, DWCP, DWP> DeleteWorkspaceOperation<'a, FWP, DWCP, DWP>
where
    FWP: FindWorkspace,
    DWCP: DeleteWorkspaceCommands,
    DWP: DeleteWorkspace,
{
    pub fn execute(&self, id: &WorkspaceId) -> Result<()> {
        tracing::info!(operation = "Delete workspace");

        self.get_workspace(id)?;

        self.delete_workspace_commands_provider
            .delete_workspace_commands(id)?;

        self.delete_workspace_provider.delete_workspace(id)
    }

    fn get_workspace(&self, id: &WorkspaceId) -> Result<Workspace> {
        GetWorkspaceOperation {
            provider: self.find_workspace_provider,
        }
        .execute(id)
    }
}
