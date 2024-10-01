use crate::{
    clients::memories::Client,
    models::workspaces::commands::get::{Model, ModelParameters},
    parameters::workspaces::commands::get::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
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
