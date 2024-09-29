use crate::{
    clients::memories,
    router::workspaces::DeleteParameters,
    routes::workspaces::list::{Model, ModelParameters},
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteParameters) -> Result<Model> {
        let DeleteParameters { id } = parameters;

        self.memories.delete_workspace(&id)?;
        let workspaces = self.memories.list_workspaces()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
