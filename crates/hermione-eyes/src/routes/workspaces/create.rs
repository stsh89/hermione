use crate::{
    clients::memories::Client,
    router::workspaces::CreateParameters,
    routes::workspaces::list::{Model, ModelParameters},
    types::{Result, Workspace},
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateParameters) -> Result<Model> {
        let CreateParameters { name, location } = parameters;

        self.memories.create_workspace(Workspace {
            id: String::new(),
            location,
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
