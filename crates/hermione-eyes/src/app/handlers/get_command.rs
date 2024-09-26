use crate::{
    clients,
    models::{GetCommandModel, GetCommandModelParameters},
    router::GetCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: GetCommandParameters) -> Result<GetCommandModel> {
        let GetCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.organizer.get_command(&workspace_id, &command_id)?;

        GetCommandModel::new(GetCommandModelParameters { command })
    }
}
