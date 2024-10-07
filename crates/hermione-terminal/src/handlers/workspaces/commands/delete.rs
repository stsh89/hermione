use crate::{
    integrations, parameters::workspaces::commands::delete::Parameters,
    presenters::workspace::Presenter, Result,
};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        self.workspaces
            .commands()
            .delete(&workspace_id, &command_id)?;
        self.workspaces.get(&workspace_id)
    }
}
