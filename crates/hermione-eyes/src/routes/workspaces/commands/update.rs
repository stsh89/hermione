use crate::{
    app::router::workspaces::commands::UpdateParameters,
    clients::memories,
    models::workspaces::commands::get::{Model, ModelParameters},
    presenters::command::Presenter,
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

        let command = Presenter {
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
