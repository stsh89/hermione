use crate::{
    integrations,
    models::workspaces::commands::list::{Model, ModelParameters},
    parameters::workspaces::commands::list::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            search_query,
        } = parameters;

        let workspace = self.workspaces.get(&workspace_id)?;

        let commands = self.workspaces.commands().list(
            integrations::core::workspaces::commands::ListParameters {
                workspace_id: &workspace_id,
                search_query: search_query.as_deref(),
            },
        )?;

        let workspace = self.workspaces.track_access_time(workspace)?;

        Model::new(ModelParameters {
            commands,
            workspace,
            search_query,
        })
    }
}
