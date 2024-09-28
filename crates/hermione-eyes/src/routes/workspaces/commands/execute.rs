use crate::{
    app::ExecuteCommandParameters,
    clients::{executor, memories},
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
            execute_immediately,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let workspace = self.memories.get_workspace(&workspace_id)?;

        let executor = executor::Client {
            program: &command.program,
            location: &workspace.location,
            execute_immediately,
        };

        executor.execute()?;
        self.memories.track_command_execution_time(command)?;

        Ok(())
    }
}
