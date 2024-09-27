use crate::{
    clients::memories,
    models::{GetWorkspaceModel, GetWorkspaceModelParameters},
    router::DeleteCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: DeleteCommandParameters) -> Result<GetWorkspaceModel> {
        let DeleteCommandParameters {
            workspace_id,
            command_id,
        } = parameters;

        self.memories.delete_command(&workspace_id, &command_id)?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;

        GetWorkspaceModel::new(GetWorkspaceModelParameters {
            workspace,
            commands_search_query: String::new(),
            commands,
        })
    }
}
