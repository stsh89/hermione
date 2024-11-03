use crate::{
    definitions::{Command, CommandId},
    operations::GetCommandOperation,
    services::{EditCommandParameters, FindCommand, UpdateCommand},
    Result,
};

pub struct UpdateCommandOperation<'a, FW, UW> {
    pub find_command_provider: &'a FW,
    pub update_command_provider: &'a UW,
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

        let mut command = GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)?;

        command.set_program(program);
        command.set_name(name);

        self.update_command_provider
            .update_command(EditCommandParameters {
                id: command.id(),
                name: command.name(),
                program: command.program(),
            })?;

        Ok(command)
    }
}
