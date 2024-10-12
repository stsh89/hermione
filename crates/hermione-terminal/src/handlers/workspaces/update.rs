use crate::{
    coordinator::Coordinator, parameters::workspaces::update::Parameters,
    presenters::workspace::Presenter, Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters { id, name, location } = parameters;

        let mut workspace = self.coordinator.workspaces().get(&id)?;

        workspace.name = name;
        workspace.location = location;

        self.coordinator.workspaces().update(workspace)
    }
}
