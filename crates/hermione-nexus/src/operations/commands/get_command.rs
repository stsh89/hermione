use eyre::eyre;

use crate::{
    definitions::{Command, CommandId},
    services::{FindCommand, StorageService},
    Error, Result,
};

pub struct GetCommandOperation<'a, SP>
where
    SP: StorageService,
{
    pub provider: &'a SP,
}

impl<F> GetCommandOperation<'_, F>
where
    F: FindCommand,
{
    pub fn execute(&self, id: CommandId) -> Result<Command> {
        tracing::info!(operation = "Get command");

        self.provider
            .find_command(id)?
            .ok_or(eyre!("Could not find command with ID: {}", id))
            .map_err(Error::not_found)
    }
}
