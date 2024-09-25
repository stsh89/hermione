use crate::{
    clients,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<ListWorkspacesModel> {
        self.organizer.delete_workspace(0)?;
        let workspaces = self.organizer.list_workspaces();

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
