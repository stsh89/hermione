use crate::{
    clients::organizer,
    entities::Command,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::CreateCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateCommandParameters) -> Result<GetWorkspaceModel> {
        let CreateCommandParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.organizer.create_command(Command {
            workspace_id: workspace_id.clone(),
            id: None,
            name,
            program,
        })?;

        let workspace = self.organizer.get_workspace(&workspace_id)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
        })
    }
}
