use crate::{
    integrations,
    models::workspaces::edit::{Model, ModelParameters},
    parameters::workspaces::edit::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        let workspace = self.workspaces.get(&id)?;

        let model = Model::new(ModelParameters { workspace });

        Ok(model)
    }
}
