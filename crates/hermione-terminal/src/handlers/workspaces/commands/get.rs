use crate::{
    integrations,
    models::workspaces::commands::get::{Model, ModelParameters},
    parameters::workspaces::commands::get::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.workspaces.commands().get(&workspace_id, &command_id)?;

        Model::new(ModelParameters { command })
    }
}
