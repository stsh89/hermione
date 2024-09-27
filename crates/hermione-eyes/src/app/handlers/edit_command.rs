use crate::{
    clients::memories,
    models::{EditCommandModel, EditCommandModelParameters},
    router::EditCommandParameters,
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditCommandParameters) -> Result<EditCommandModel> {
        let EditCommandParameters {
            command_id,
            workspace_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        Ok(EditCommandModel::new(EditCommandModelParameters {
            command,
        }))
    }
}
