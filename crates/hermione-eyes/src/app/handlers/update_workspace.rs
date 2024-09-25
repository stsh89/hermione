use crate::{
    clients,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::UpdateWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: UpdateWorkspaceParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetWorkspaceModel> {
        let UpdateWorkspaceParameters { name, location } = self.parameters;
        self.organizer.rename_workspace(0, name.to_string())?;
        self.organizer.set_workspace_location(0, location)?;

        let workspace = self.organizer.get_workspace(0)?;

        let model = GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })?;

        Ok(model)
    }
}
