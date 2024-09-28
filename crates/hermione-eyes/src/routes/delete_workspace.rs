use crate::{
    app::DeleteWorkspaceParameters,
    clients::memories,
    routes::list_workspaces::{Model, ModelParameters},
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteWorkspaceParameters) -> Result<Model> {
        let DeleteWorkspaceParameters { id } = parameters;

        self.memories.delete_workspace(&id)?;
        let workspaces = self.memories.list_workspaces()?;

        let model = Model::new(ModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
