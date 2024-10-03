use crate::{
    clients::memories::{self, WorkspacesCommandsListParameters},
    models::workspaces::commands::list::{Model, ModelParameters},
    parameters::workspaces::commands::delete::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        self.memories.delete_command(&workspace_id, &command_id)?;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self
            .memories
            .list_commands(WorkspacesCommandsListParameters {
                workspace_id: &workspace_id,
                ..Default::default()
            })?;

        Model::new(ModelParameters {
            workspace,
            search_query: None,
            commands,
        })
    }
}
