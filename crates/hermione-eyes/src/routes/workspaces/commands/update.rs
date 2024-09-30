use crate::{
    clients::memories,
    presenters::Command,
    router::workspaces::commands::UpdateParameters,
    routes::workspaces::commands::get::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateParameters) -> Result<Model> {
        let UpdateParameters {
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
