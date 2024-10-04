use crate::{
    clients::memories::{Client, WorkspacesCommandsListParameters},
    models::workspaces::commands::list::{Model, ModelParameters},
    parameters::workspaces::commands::list::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            search_query,
        } = parameters;

        let workspace = self.memories.get_workspace(&workspace_id)?;

        let commands = self
            .memories
            .list_commands(WorkspacesCommandsListParameters {
                workspace_id: &workspace_id,
                search_query: search_query.as_deref(),
            })?;

        let workspace = self.memories.track_workspace_access_time(workspace)?;

        Model::new(ModelParameters {
            commands,
            workspace,
            search_query,
        })
    }
}
