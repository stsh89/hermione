use crate::{
    coordinator::Coordinator,
    models::workspaces::commands::get::{Model, ModelParameters},
    parameters::workspaces::commands::update::Parameters,
    presenters::command::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
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

        let command = self.coordinator.workspaces().commands().update(command)?;
        let workspace = self.coordinator.workspaces().get(&command.workspace_id)?;

        let model = Model::new(ModelParameters { command, workspace })?;

        Ok(model)
    }
}
