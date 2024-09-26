use crate::{
    clients::{executor, organizer},
    router::ExecuteCommandParameters,
    Result,
};

pub struct Handler<'a> {
    pub parameters: ExecuteCommandParameters,
    pub organizer: &'a mut organizer::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self) -> Result<()> {
        let ExecuteCommandParameters { number } = self.parameters;
        let command = self.organizer.get_command(0, number)?;
        let workspace = self.organizer.get_workspace(0)?;

        let executor = executor::Client::new(&command, &workspace.location);
        executor.execute()
    }
}
