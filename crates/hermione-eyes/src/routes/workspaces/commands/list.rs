use crate::{
    app::router::workspaces::commands::ListParameters,
    clients::memories::Client,
    models::workspaces::commands::list::{Model, ModelParameters},
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ListParameters) -> Result<Model> {
        let ListParameters {
            workspace_id,
            search_query,
        } = parameters;

        let workspace = self.memories.get_workspace(&workspace_id)?;
        let commands = self.memories.list_commands(&workspace_id)?;
        let filter = search_query.to_lowercase();

        let commands = if !filter.is_empty() {
            commands
                .into_iter()
                .filter(|c| c.program.to_lowercase().contains(&filter))
                .collect()
        } else {
            commands
        };

        let workspace = self.memories.track_workspace_access_time(workspace)?;

        Model::new(ModelParameters {
            commands,
            workspace,
            search_query,
        })
    }
}
