use crate::{
    app::UpdateCommandParameters,
    clients::memories,
    routes::workspaces::commands::get::{Model, ModelParameters},
    types::{Command, Result},
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateCommandParameters) -> Result<Model> {
        let UpdateCommandParameters {
            command_id,
            workspace_id,
            name,
            program,
        } = parameters;

        let command = Command {
            workspace_id,
            id: command_id.clone(),
            name,
            program,
        };

        let command = self.memories.update_command(command)?;

        let model = Model::new(ModelParameters { command })?;

        Ok(model)
    }
}