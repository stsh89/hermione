use crate::{app::router::powershell::CopyToClipboardParameters, clients::memories, Result};
use hermione_wand::clients::powershell;

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: CopyToClipboardParameters) -> Result<()> {
        let CopyToClipboardParameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.memories.get_command(&workspace_id, &command_id)?;
        let powershell = powershell::Client::new()?;

        powershell.copy_to_clipboard(&command.program)
    }
}
