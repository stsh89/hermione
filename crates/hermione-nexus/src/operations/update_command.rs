use crate::{
    definitions::{Command, CommandId},
    services::{EditCommandParameters, FindCommand, UpdateCommand},
    Error, Result,
};

pub struct UpdateCommandOperation<'a, FW, UW> {
    pub find_provider: &'a FW,
    pub update_provider: &'a UW,
}

pub struct UpdateCommandParameters<'a> {
    pub id: &'a CommandId,
    pub program: String,
    pub name: String,
}

impl<'a, FW, UW> UpdateCommandOperation<'a, FW, UW>
where
    FW: FindCommand,
    UW: UpdateCommand,
{
    pub fn execute(&self, parameters: UpdateCommandParameters) -> Result<Command> {
        tracing::info!(operation = "Update command");

        let UpdateCommandParameters { id, program, name } = parameters;

        let Some(mut command) = self.find_provider.find_command(id)? else {
            return Err(Error::NotFound(format!("Command with ID: {}", **id)));
        };

        command.set_program(program);
        command.set_name(name);

        self.update_provider.update_command(EditCommandParameters {
            id: command.id(),
            name: command.name(),
            program: command.program(),
        })?;

        Ok(command)
    }
}
