use crate::{
    coordinator::{workspaces::commands::ListParameters, Coordinator},
    models::workspaces::commands::list::{Model, ModelParameters},
    parameters::workspaces::commands::list::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            workspace_id,
            search_query,
        } = parameters;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        let commands = self
            .coordinator
            .workspaces()
            .commands()
            .list(ListParameters {
                workspace_id: &workspace_id,
                program_contains: search_query.as_deref(),
            })?;

        let workspace = self.coordinator.workspaces().track_access_time(workspace)?;

        Model::new(ModelParameters {
            commands,
            workspace,
            search_query,
        })
    }
}
