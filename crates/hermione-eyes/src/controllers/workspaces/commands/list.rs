use crate::{
    clients::memories::Client,
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
        let commands = self.memories.list_commands(&workspace.id)?;
        let filter = search_query.as_ref().map(|query| query.to_lowercase());

        let commands = if let Some(filter) = filter {
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
