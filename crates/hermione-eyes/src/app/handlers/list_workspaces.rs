use crate::{
    clients,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::ListWorkspacesParameters,
};

pub struct Handler<'a> {
    pub parameters: ListWorkspacesParameters,
    pub organizer: &'a clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> ListWorkspacesModel {
        let ListWorkspacesParameters { search_query } = self.parameters;
        let mut workspaces = self.organizer.list_workspaces();
        let filter = search_query.to_lowercase();

        if !filter.is_empty() {
            workspaces = workspaces
                .into_iter()
                .filter(|w| w.name.to_lowercase().contains(&filter))
                .collect();
        }

        ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query,
        })
    }
}
