use crate::{
    clients::{memories, powershell},
    parameters::powershell::copy_to_clipboard::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let powershell = powershell::Client::new()?;

        powershell.copy_to_clipboard(&command.program)
    }
}