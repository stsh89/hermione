use crate::{
    integrations,
    models::workspaces::commands::new::{Model, ModelParameters},
    parameters::workspaces::commands::new::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { workspace_id } = parameters;

        let workspace = self.workspaces.get(&workspace_id)?;

        Ok(Model::new(ModelParameters { workspace }))
    }
}
