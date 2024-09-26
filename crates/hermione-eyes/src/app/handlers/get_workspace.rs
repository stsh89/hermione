use crate::{
    clients,
    entities::Workspace,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::GetWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: GetWorkspaceParameters) -> Result<GetWorkspaceModel> {
        let GetWorkspaceParameters {
            id,
            commands_search_query,
        } = parameters;

        let workspace = self.organizer.get_workspace(&id)?;
        let filter = commands_search_query.to_lowercase();

        let commands = if !filter.is_empty() {
            workspace
                .commands
                .into_iter()
                .filter(|c| c.program.to_lowercase().contains(&filter))
                .collect()
        } else {
            workspace.commands
        };

        let workspace = Workspace {
            commands,
            ..workspace
        };

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query,
        })
    }
}
