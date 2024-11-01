use crate::{
    services::storage::{CreateWorkspace, CreateWorkspaceParameters, Workspace},
    Result,
};

pub struct CreateWorkspaceOperation<'a, CW> {
    pub operator: &'a CW,
}

pub struct CreateWorkspaceOperand {
    pub name: String,
    pub location: Option<String>,
}

impl<'a, CW> CreateWorkspaceOperation<'a, CW>
where
    CW: CreateWorkspace,
{
    pub fn execute(&self, operand: CreateWorkspaceOperand) -> Result<Workspace> {
        tracing::info!(operation = "Create workspace");

        let CreateWorkspaceOperand { name, location } = operand;

        self.operator
            .create_workspace(CreateWorkspaceParameters { name, location })
    }
}
