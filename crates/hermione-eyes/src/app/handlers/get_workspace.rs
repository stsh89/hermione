use crate::{
    clients,
    entities::Workspace,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::GetWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a mut clients::organizer::Client,
    pub parameters: GetWorkspaceParameters,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<GetWorkspaceModel> {
        let GetWorkspaceParameters {
            number,
            commands_search_query,
        } = self.parameters;

        let workspace = self.organizer.get_workspace(number)?;
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

        self.organizer.promote_workspace(workspace.number)?;

        let model = GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query,
        });

        Ok(model)
    }
}
