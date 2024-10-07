use crate::{
    brokers, clients::memories, parameters::powershell::copy_to_clipboard::Parameters, Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
    pub powershell: &'a brokers::powershell::Broker,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;

        self.powershell.copy_to_clipboard(&command.program)
    }
}
