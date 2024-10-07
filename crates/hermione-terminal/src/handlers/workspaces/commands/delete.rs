use crate::{
    coordinator::Coordinator, parameters::workspaces::commands::delete::Parameters,
    presenters::workspace::Presenter, Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        self.coordinator
            .workspaces()
            .commands()
            .delete(&workspace_id, &command_id)?;
        self.coordinator.workspaces().get(&workspace_id)
    }
}
