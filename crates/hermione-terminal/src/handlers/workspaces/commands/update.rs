use crate::{
    integrations,
    models::workspaces::commands::get::{Model, ModelParameters},
    parameters::workspaces::commands::update::Parameters,
    presenters::command::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
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

        let command = self.workspaces.commands().update(command)?;

        let model = Model::new(ModelParameters { command })?;

        Ok(model)
    }
}
