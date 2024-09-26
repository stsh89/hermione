use crate::{
    clients,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::DeleteWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let DeleteWorkspaceParameters { id } = parameters;

        self.organizer.delete_workspace(&id)?;
        let workspaces = self.organizer.list_workspaces()?;

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
