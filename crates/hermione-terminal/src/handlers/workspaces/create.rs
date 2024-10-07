use crate::{
    integrations,
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::create::Parameters,
    presenters::workspace::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { name, location } = parameters;

        self.workspaces.create(Presenter {
            id: String::new(),
            location: Some(location),
            name,
        })?;

        let workspaces = self.workspaces.list()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
