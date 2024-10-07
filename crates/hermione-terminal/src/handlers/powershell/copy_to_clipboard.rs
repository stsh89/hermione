use crate::{integrations, parameters::powershell::copy_to_clipboard::Parameters, Result};

pub struct Handler<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self.workspaces.commands().get(&workspace_id, &command_id)?;
        let powershell = integrations::powershell::Client::new()?;

        powershell.copy_to_clipboard(&command.program)
    }
}
