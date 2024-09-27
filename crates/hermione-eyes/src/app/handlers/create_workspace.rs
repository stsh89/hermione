use crate::{
    clients::memories,
    entities::Workspace,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::CreateWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let CreateWorkspaceParameters { name, location } = parameters;

        self.memories.create_workspace(Workspace {
            id: None,
            location,
            name,
        })?;

        let workspaces = self.memories.list_workspaces()?;

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
