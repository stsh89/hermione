use crate::{
    clients::organizer,
    models::{EditCommandModel, EditCommandModelParameters},
    router::EditCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditCommandParameters) -> Result<EditCommandModel> {
        let EditCommandParameters {
            command_id,
            workspace_id,
        } = parameters;

        let command = self.organizer.get_command(&workspace_id, &command_id)?;

        Ok(EditCommandModel::new(EditCommandModelParameters {
            command,
        }))
    }
}
