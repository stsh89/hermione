use crate::{
    clients::{executor, memories},
    router::ExecuteCommandParameters,
    types::Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ExecuteCommandParameters) -> Result<()> {
        let ExecuteCommandParameters {
            workspace_id,
            command_id,
        } = parameters;
        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let workspace = self.memories.get_workspace(&workspace_id)?;

        let executor = executor::Client::new(&command, &workspace.location);
        executor.execute()?;

        Ok(())
    }
}
