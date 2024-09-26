use crate::{
    clients::{executor, organizer},
    router::ExecuteCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub organizer: &'a organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ExecuteCommandParameters) -> Result<()> {
        let ExecuteCommandParameters {
            workspace_id,
            command_id,
        } = parameters;
        let command = self.organizer.get_command(&workspace_id, &command_id)?;
        let workspace = self.organizer.get_workspace(&workspace_id)?;

        let executor = executor::Client::new(&command, &workspace.location);
        executor.execute()?;

        Ok(())
    }
}
