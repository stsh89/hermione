use crate::{
    clients,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::CreateWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: CreateWorkspaceParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<ListWorkspacesModel> {
        let CreateWorkspaceParameters { name } = self.parameters;
        self.organizer.add_workspace(name.to_string())?;

        let workspaces = self.organizer.list_workspaces();

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
