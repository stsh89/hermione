use crate::{
    services::storage::{
        EditWorkspaceParameters, FindWorkspace, UpdateWorkspace, Workspace, WorkspaceId,
    },
    Error, Result,
};

pub struct UpdateWorkspaceOperation<'a, FW, UW> {
    pub find_operator: &'a FW,
    pub update_operator: &'a UW,
}

pub struct UpdateWorkspaceParameters<'a> {
    pub id: &'a WorkspaceId,
    pub location: Option<String>,
    pub name: String,
}

impl<'a, FW, UW> UpdateWorkspaceOperation<'a, FW, UW>
where
    FW: FindWorkspace,
    UW: UpdateWorkspace,
{
    pub fn execute(&self, parameters: UpdateWorkspaceParameters) -> Result<Workspace> {
        tracing::info!(operation = "Update workspace");

        let UpdateWorkspaceParameters { id, location, name } = parameters;

        let Some(mut workspace) = self.find_operator.find_workspace(id)? else {
            return Err(Error::NotFound(format!("Workspace with ID: {}", **id)));
        };

        workspace.set_location(location);
        workspace.set_name(name);

        self.update_operator
            .update_workspace(EditWorkspaceParameters {
                id: workspace.id(),
                name: workspace.name(),
                location: workspace.location(),
            })?;

        Ok(workspace)
    }
}
