use crate::{
    definitions::WorkspaceId,
    operations::GetWorkspaceOperation,
    services::{
        FindWorkspace, InvokeCommand, InvokeCommandParameters, StorageService, SystemService,
    },
    Result,
};

pub struct ExecuteProgramOperation<'a, FW, S>
where
    FW: StorageService,
    S: SystemService,
{
    pub system: &'a S,
    pub find_workspace: &'a FW,
}

pub struct ExecuteProgramParameters<'a> {
    pub program: &'a str,
    pub workspace_id: Option<WorkspaceId>,
}

impl<FW, S> ExecuteProgramOperation<'_, FW, S>
where
    FW: FindWorkspace,
    S: InvokeCommand,
{
    pub fn execute(&self, parameters: ExecuteProgramParameters) -> Result<()> {
        let ExecuteProgramParameters {
            program: command,
            workspace_id,
        } = parameters;

        if let Some(id) = workspace_id {
            let workspace = GetWorkspaceOperation {
                provider: self.find_workspace,
            }
            .execute(id)?;

            self.system.invoke_command(InvokeCommandParameters {
                command,
                location: workspace.location(),
            })?;
        } else {
            self.system.invoke_command(InvokeCommandParameters {
                command,
                location: None,
            })?;
        };

        Ok(())
    }
}
