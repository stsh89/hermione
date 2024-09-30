use crate::{
    app::router::workspaces::commands::GetParameters,
    clients::memories::Client,
    models::workspaces::commands::get::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: GetParameters) -> Result<Model> {
        let GetParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        Model::new(ModelParameters { command })
    }
}
