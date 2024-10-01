use crate::{
    clients::memories::Client,
    models::workspaces::commands::get::{Model, ModelParameters},
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
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        Model::new(ModelParameters { command })
    }
}
