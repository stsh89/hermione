use crate::{
    definitions::{Command, CommandId, WorkspaceId},
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
    pub workspace_id: &'a WorkspaceId,
}

impl<'a, FW, UW> UpdateCommandOperation<'a, FW, UW>
where
    FW: FindCommand,
    UW: UpdateCommand,
{
    pub fn execute(&self, parameters: UpdateCommandParameters) -> Result<Command> {
        tracing::info!(operation = "Update command");

        let UpdateCommandParameters {
            id,
            program,
            name,
            workspace_id,
        } = parameters;

        let Some(mut command) = self.find_provider.find_command(id)? else {
            return Err(Error::NotFound(format!("Command with ID: {}", **id)));
        };

        if **command.workspace_id() != **workspace_id {
            return Err(Error::InvalidArgument(
                "Command does not belong to workspace".to_string(),
            ));
        }

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
