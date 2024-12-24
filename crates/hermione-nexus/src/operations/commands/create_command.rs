use crate::{
    definitions::{Command, WorkspaceId},
    services::{CreateCommand, NewCommandParameters, StorageService},
    Result,
};

pub struct CreateCommandOperation<'a, SP>
where
    SP: StorageService,
{
    pub storage_provider: &'a SP,
}

pub struct CreateCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}

impl<CC> CreateCommandOperation<'_, CC>
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

        self.storage_provider.create_command(NewCommandParameters {
            name,
            program,
            workspace_id,
        })
    }
}
