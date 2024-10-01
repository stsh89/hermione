use crate::{
    clients::memories::Client,
    models::workspaces::commands::edit::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

pub struct Parameters {
    pub command_id: String,
    pub workspace_id: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            command_id,
            workspace_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        Ok(Model::new(ModelParameters { command }))
    }
}
