use crate::{
    clients,
    entities::Command,
    models::{GetCommandModel, GetCommandModelParameters},
    router::UpdateCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a clients::organizer::Client,
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

        self.organizer.update_command(&command)?;

        let model = GetCommandModel::new(GetCommandModelParameters { command })?;

        Ok(model)
    }
}
