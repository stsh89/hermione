use crate::{
    clients,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::UpdateWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: UpdateWorkspaceParameters) -> Result<GetWorkspaceModel> {
        let UpdateWorkspaceParameters { id, name, location } = parameters;

        let mut workspace = self.organizer.get_workspace(&id)?;

        workspace.name = name;
        workspace.location = location;

        self.organizer.update_workspace(&workspace)?;

        let model = GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })?;

        Ok(model)
    }
}
