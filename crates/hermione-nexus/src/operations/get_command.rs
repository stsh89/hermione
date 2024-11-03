use crate::{
    definitions::{Command, CommandId},
    services::FindCommand,
    Error, Result,
};

pub struct GetCommandOperation<'a, F> {
    pub provider: &'a F,
}

impl<'a, F> GetCommandOperation<'a, F>
where
    F: FindCommand,
{
    pub fn execute(&self, id: &CommandId) -> Result<Command> {
        tracing::info!(operation = "Get command");

        let Some(command) = self.provider.find_command(id)? else {
            return Err(Error::NotFound(format!("Command {}", id.braced())));
        };

        Ok(command)
    }
}
