use crate::{
    coordinator::{self, Coordinator},
    models::workspaces::commands::list::{Model, ModelParameters},
    parameters::{self, workspaces::commands::update::Parameters},
    presenters::command::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Model> {
        let Parameters {
            command_id,
            workspace_id,
            name,
            program,
        } = parameters;

        let command = Presenter {
            workspace_id,
            id: command_id.clone(),
            name,
            program,
        };

        let command = self.coordinator.workspaces().commands().update(command)?;
        let workspace = self.coordinator.workspaces().get(&command.workspace_id)?;
        let commands = self.coordinator.workspaces().commands().list(
            coordinator::workspaces::commands::ListParameters {
                page_number: 0,
                page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                program_contains: &command.program,
                workspace_id: &workspace.id,
            },
        )?;

        let model = Model::new(ModelParameters {
            commands,
            workspace,
            search_query: command.program,
            page_number: 0,
            page_size: parameters::workspaces::commands::list::PAGE_SIZE,
        })?;

        Ok(model)
    }
}
