use crate::{
    app::CreateWorkspaceParameters,
    clients::memories,
    routes::list_workspaces::{Model, ModelParameters},
    types::{Result, Workspace},
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateWorkspaceParameters) -> Result<Model> {
        let CreateWorkspaceParameters { name, location } = parameters;

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
