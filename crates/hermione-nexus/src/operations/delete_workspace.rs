use crate::{
    definitions::WorkspaceId,
    services::{DeleteWorkspace, DeleteWorkspaceCommands, FindWorkspace},
    Error, Result,
};

use super::GetWorkspaceOperation;

pub struct DeleteWorkspaceOperation<'a, FWP, DWCP, DWP> {
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
        GetWorkspaceOperation {
            provider: self.find_workspace_provider,
        }.execute(id)?;

        self.delete_workspace_commands_provider
            .delete_workspace_commands(id)?;

        self.delete_workspace_provider.delete_workspace(id)?;

        Ok(())
    }
}
