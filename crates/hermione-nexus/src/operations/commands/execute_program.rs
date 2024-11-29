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
    pub workspace_id: WorkspaceId,
}

impl<'a, FW, S> ExecuteProgramOperation<'a, FW, S>
where
    FW: FindWorkspace,
    S: InvokeCommand,
{
    pub fn execute(&self, parameters: ExecuteProgramParameters) -> Result<()> {
        let ExecuteProgramParameters {
            program,
            workspace_id,
        } = parameters;

        let workspace = GetWorkspaceOperation {
            provider: self.find_workspace,
        }
        .execute(workspace_id)?;

        self.system.invoke_command(InvokeCommandParameters {
            command: program,
            location: workspace.location(),
        })
    }
}
