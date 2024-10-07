use crate::{
    integrations, parameters::workspaces::update::Parameters, presenters::workspace::Presenter,
    Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters { id, name, location } = parameters;

        let mut workspace = self.workspaces.get(&id)?;

        workspace.name = name;
        workspace.location = Some(location);

        self.workspaces.update(workspace)
    }
}
