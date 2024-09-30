use crate::{
    app::router::workspaces::commands::EditParameters,
    clients::memories::Client,
    models::workspaces::commands::edit::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditParameters) -> Result<Model> {
        let EditParameters {
            command_id,
            workspace_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        Ok(Model::new(ModelParameters { command }))
    }
}
