use crate::{
    clients::organizer,
    entities::Workspace,
    models::{ListWorkspacesModel, ListWorkspacesModelParameters},
    router::CreateWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateWorkspaceParameters) -> Result<ListWorkspacesModel> {
        let CreateWorkspaceParameters { name, location } = parameters;

        self.organizer.create_workspace(Workspace {
            commands: vec![],
            id: None,
            location,
            name,
        })?;

        let workspaces = self.organizer.list_workspaces()?;

        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
        })?;

        Ok(model)
    }
}
