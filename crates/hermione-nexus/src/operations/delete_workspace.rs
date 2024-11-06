use crate::{
    definitions::WorkspaceId,
    services::{DeleteWorkspace, DeleteWorkspaceCommands, FindWorkspace, StorageService},
    Error, Result,
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
        if self.find_workspace_provider.find_workspace(id)?.is_none() {
            return Err(Error::NotFound(format!("Workspace {}", id.braced())));
        }

        self.delete_workspace_commands_provider
            .delete_workspace_commands(id)?;

        self.delete_workspace_provider.delete_workspace(id)?;

        Ok(())
    }
}
