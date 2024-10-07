use crate::{
    integrations,
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::delete::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        self.workspaces.delete(&id)?;
        let workspaces = self.workspaces.list()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
