use crate::{
    clients::memories,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::GetWorkspaceParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: GetWorkspaceParameters) -> Result<GetWorkspaceModel> {
        let GetWorkspaceParameters {
            id,
            commands_search_query,
        } = parameters;

        let workspace = self.memories.get_workspace(&id)?;
        let commands = self.memories.list_commands(&id)?;
        let filter = commands_search_query.to_lowercase();

        let commands = if !filter.is_empty() {
            commands
                .into_iter()
                .filter(|c| c.program.to_lowercase().contains(&filter))
                .collect()
        } else {
            commands
        };

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            commands,
            workspace,
            commands_search_query,
        })
    }
}
