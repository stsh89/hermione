use crate::{
    definitions::{Command, CommandId},
    operations::GetCommandOperation,
    services::{EditCommandParameters, FindCommand, StorageService, UpdateCommand},
    Result,
};

pub struct UpdateCommandOperation<'a, FW, UW>
where
    FW: StorageService,
    UW: StorageService,
{
    pub find_command_provider: &'a FW,
    pub update_command_provider: &'a UW,
}

pub struct UpdateCommandParameters {
    pub id: CommandId,
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

        let mut command = self.get_command(id)?;

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

    fn get_command(&self, id: CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)
    }
}
