use crate::{
    clients::memories,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::DeleteWorkspaceParameters,
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let DeleteWorkspaceParameters { id } = parameters;

        self.memories.delete_workspace(&id)?;
        let workspaces = self.memories.list_workspaces()?;

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
