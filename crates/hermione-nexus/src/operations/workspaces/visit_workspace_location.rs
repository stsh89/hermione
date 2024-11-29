use crate::{
    definitions::WorkspaceId,
    services::{FindWorkspace, SetLocation, StorageService, SystemService},
    Result,
};

use super::GetWorkspaceOperation;

pub struct VisitWorkspaceLocationOperation<'a, FW, S>
where
    S: SystemService,
    FW: StorageService,
{
    pub find_workspace: &'a FW,
    pub system_provider: &'a S,
}

impl<'a, FW, S> VisitWorkspaceLocationOperation<'a, FW, S>
where
    FW: FindWorkspace,
    S: SetLocation,
{
    pub fn execute(&self, id: WorkspaceId) -> Result<()> {
        let workspace = GetWorkspaceOperation {
            provider: self.find_workspace,
        }
        .execute(id)?;

        self.system_provider.set_location(workspace.location())
    }
}
