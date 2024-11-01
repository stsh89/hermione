use crate::{
    services::storage::{FindWorkspace, Workspace, WorkspaceId},
    Error, Result,
};

pub struct GetWorkspaceOperation<'a, F> {
    pub operator: &'a F,
}

impl<'a, F> GetWorkspaceOperation<'a, F>
where
    F: FindWorkspace,
{
    pub fn execute(&self, id: &WorkspaceId) -> Result<Workspace> {
        tracing::info!(operation = "Get workspace");

        let Some(workspace) = self.operator.find_workspace(id)? else {
            return Err(Error::NotFound(format!("Workspace with ID: {}", **id)));
        };

        Ok(workspace)
    }
}
