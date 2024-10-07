use crate::{
    clients, parameters::workspaces::commands::delete::Parameters,
    presenters::workspace::Presenter, Result,
};

pub struct Handler<'a> {
    pub memories: &'a clients::memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<Presenter> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        self.memories.delete_command(&workspace_id, &command_id)?;
        self.memories.get_workspace(&workspace_id)
    }
}
