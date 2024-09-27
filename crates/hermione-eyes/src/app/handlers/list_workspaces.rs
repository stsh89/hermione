use crate::{
    clients::memories,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::ListWorkspacesParameters,
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ListWorkspacesParameters) -> Result<ListWorkspacesModel> {
        let ListWorkspacesParameters { search_query } = parameters;
        let mut workspaces = self.memories.list_workspaces()?;
        let filter = search_query.to_lowercase();

        if !filter.is_empty() {
            workspaces.retain(|w| w.name.to_lowercase().contains(&filter));
        }

        ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query,
        })
    }
}
