use crate::{
    services::storage::{
        FindWorkspace, UpdateWorkspace, UpdateWorkspaceParameters, Workspace, WorkspaceId,
    },
    Error, Result,
};

pub struct UpdateWorkspaceOperation<'a, FW, UW> {
    pub find_operator: &'a FW,
    pub update_operator: &'a UW,
}

pub struct UpdateWorkspaceOperand<'a> {
    pub id: &'a WorkspaceId,
    pub location: Option<String>,
    pub name: String,
}

impl<'a, FW, UW> UpdateWorkspaceOperation<'a, FW, UW>
where
    FW: FindWorkspace,
    UW: UpdateWorkspace,
{
    pub fn execute(&self, operand: UpdateWorkspaceOperand) -> Result<Workspace> {
        tracing::info!(operation = "Update workspace");

        let UpdateWorkspaceOperand { id, location, name } = operand;

        let Some(mut workspace) = self.find_operator.find_workspace(id)? else {
            return Err(Error::NotFound(format!("Workspace with ID: {}", **id)));
        };

        workspace.set_location(location);
        workspace.set_name(name);

        self.update_operator
            .update_workspace(UpdateWorkspaceParameters {
                id: workspace.id(),
                name: workspace.name(),
                location: workspace.location(),
            })?;

        Ok(workspace)
    }
}
