use crate::{
    definitions::{Command, WorkspaceId},
    services::{CreateCommand, NewCommandParameters},
    Result,
};

pub struct CreateCommandOperation<'a, CW> {
    pub provider: &'a CW,
}

pub struct CreateCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}

impl<'a, CW> CreateCommandOperation<'a, CW>
where
    CW: CreateCommand,
{
    pub fn execute(&self, parameters: CreateCommandParameters) -> Result<Command> {
        tracing::info!(operation = "Create command");

        let CreateCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        self.provider.create_command(NewCommandParameters {
            name,
            program,
            workspace_id,
        })
    }
}
