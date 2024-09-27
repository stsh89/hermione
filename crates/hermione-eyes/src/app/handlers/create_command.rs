use crate::{
    clients::memories,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::CreateCommandParameters,
    types::{Command, Result},
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CreateCommandParameters) -> Result<GetWorkspaceModel> {
        let CreateCommandParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.memories.create_command(Command {
            workspace_id: workspace_id.clone(),
            id: None,
            name,
            program,
        })?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands,
            commands_search_query: String::new(),
        })
    }
}
