use crate::{
    clients, parameters::workspaces::update::Parameters, presenters::workspace::Presenter, Result,
};

pub struct Handler<'a> {
    pub memories: &'a clients::memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters { id, name, location } = parameters;

        let mut workspace = self.memories.get_workspace(&id)?;

        workspace.name = name;
        workspace.location = Some(location);

        self.memories.update_workspace(workspace)
    }
}
