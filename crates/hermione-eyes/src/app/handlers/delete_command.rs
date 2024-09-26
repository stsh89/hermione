use crate::{
    clients::organizer,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::DeleteCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteCommandParameters) -> Result<GetWorkspaceModel> {
        let DeleteCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        self.organizer.delete_command(&workspace_id, &command_id)?;

        let workspace = self.organizer.get_workspace(&workspace_id)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })
    }
}
