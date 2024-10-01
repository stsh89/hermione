use crate::{
    clients::memories,
    models::workspaces::list::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Parameters {
    pub id: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters { id } = parameters;

        self.memories.delete_workspace(&id)?;
        let workspaces = self.memories.list_workspaces()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
