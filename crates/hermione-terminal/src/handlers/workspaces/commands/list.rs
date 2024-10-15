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
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace_id,
        } = parameters;

        let workspace = self.coordinator.workspaces().get(&workspace_id)?;

        let commands = self
            .coordinator
            .workspaces()
            .commands()
            .list(ListParameters {
                workspace_id: &workspace_id,
                program_contains: &search_query,
                page_number,
                page_size,
            })?;

        let workspace = self.coordinator.workspaces().track_access_time(workspace)?;

        Model::new(ModelParameters {
            commands,
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace,
        })
    }
}
