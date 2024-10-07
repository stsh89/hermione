use crate::{
    integrations,
    models::workspaces::commands::edit::{Model, ModelParameters},
    parameters::workspaces::commands::edit::Parameters,
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
        } = parameters;

        let command = self.workspaces.commands().get(&workspace_id, &command_id)?;

        Ok(Model::new(ModelParameters { command }))
    }
}
