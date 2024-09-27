use crate::{
    clients::memories,
    entities::Command,
    models::{GetCommandModel, GetCommandModelParameters},
    router::UpdateCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateCommandParameters) -> Result<GetCommandModel> {
        let UpdateCommandParameters {
            command_id,
            workspace_id,
            name,
            program,
        } = parameters;

        let command = Command {
            workspace_id,
            id: Some(command_id.clone()),
            name,
            program,
        };

        let command = self.memories.update_command(command)?;

        let model = GetCommandModel::new(GetCommandModelParameters { command })?;

        Ok(model)
    }
}
