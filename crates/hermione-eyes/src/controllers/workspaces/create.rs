use crate::{
    clients::memories::Client,
    models::workspaces::list::{Model, ModelParameters},
    parameters::workspaces::create::Parameters,
    presenters::workspace::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { name, location } = parameters;

        self.memories.create_workspace(Presenter {
            id: String::new(),
            location: Some(location),
            name,
        })?;

        let workspaces = self.memories.list_workspaces()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
