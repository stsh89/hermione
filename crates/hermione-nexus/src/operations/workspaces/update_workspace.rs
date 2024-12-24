use crate::{
    definitions::{Workspace, WorkspaceId},
    operations::GetWorkspaceOperation,
    services::{EditWorkspaceParameters, FindWorkspace, StorageService, UpdateWorkspace},
    Result,
};

pub struct UpdateWorkspaceOperation<'a, FW, UW>
where
    FW: StorageService,
    UW: StorageService,
{
    pub find_workspace_provider: &'a FW,
    pub update_workspace_provider: &'a UW,
}

pub struct UpdateWorkspaceParameters {
    pub id: WorkspaceId,
    pub location: Option<String>,
    pub name: String,
}

impl<FW, UW> UpdateWorkspaceOperation<'_, FW, UW>
where
    FW: FindWorkspace,
    UW: UpdateWorkspace,
{
    pub fn execute(&self, parameters: UpdateWorkspaceParameters) -> Result<Workspace> {
        tracing::info!(operation = "Update workspace");

        let UpdateWorkspaceParameters { id, location, name } = parameters;

        let mut workspace = self.get_workspace(id)?;

        workspace.set_location(location);
        workspace.set_name(name);

        self.update_workspace_provider
            .update_workspace(EditWorkspaceParameters {
                id: workspace.id(),
                name: workspace.name(),
                location: workspace.location(),
            })?;

        Ok(workspace)
    }

    fn get_workspace(&self, id: WorkspaceId) -> Result<Workspace> {
        GetWorkspaceOperation {
            provider: self.find_workspace_provider,
        }
        .execute(id)
    }
}
