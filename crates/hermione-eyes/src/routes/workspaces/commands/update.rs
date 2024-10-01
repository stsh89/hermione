use crate::{
    clients::memories,
    models::workspaces::commands::get::{Model, ModelParameters},
    presenters::command::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Parameters {
    pub command_id: String,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
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
