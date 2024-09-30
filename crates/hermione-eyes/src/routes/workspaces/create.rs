use crate::{
    app::router::workspaces::CreateParameters,
    clients::memories::Client,
    models::workspaces::list::{Model, ModelParameters},
    presenters::workspace::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateParameters) -> Result<Model> {
        let CreateParameters { name, location } = parameters;

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
