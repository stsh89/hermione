use crate::{
    definitions::{Command, CommandId},
    services::{FindCommand, StorageProvider},
    Error, Result,
};

pub struct GetCommandOperation<'a, SP>
where
    SP: StorageProvider,
{
    pub provider: &'a SP,
}

impl<'a, F> GetCommandOperation<'a, F>
where
    F: FindCommand,
{
    pub fn execute(&self, id: &CommandId) -> Result<Command> {
        tracing::info!(operation = "Get command");

        self.provider
            .find_command(id)?
            .ok_or(Error::NotFound(format!("Command {}", id.braced())))
    }
}
