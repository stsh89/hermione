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
        let CreateWorkspaceParameters { name, location } = self.parameters;
        let workspace = self.organizer.add_workspace(name.to_string())?;
        self.organizer
            .set_workspace_location(workspace.number, location)?;

        let workspaces = self.organizer.list_workspaces();

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
