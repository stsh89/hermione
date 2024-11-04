use crate::{
    definitions::{Command, WorkspaceId},
    services::{CreateCommand, NewCommandParameters, StorageProvider},
    Result,
};

pub struct CreateCommandOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

pub struct CreateCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}

impl<'a, CC> CreateCommandOperation<'a, CC>
where
    CC: CreateCommand,
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
