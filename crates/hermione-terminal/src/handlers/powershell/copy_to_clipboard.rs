use crate::{
    clients, coordinator::Coordinator, parameters::powershell::copy_to_clipboard::Parameters,
    Result,
};

pub struct Handler<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a clients::powershell::PowerShell,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: Parameters) -> Result<()> {
        let Parameters {
            workspace_id,
            command_id,
        } = parameters;

        let command = self
            .coordinator
            .workspaces()
            .commands()
            .get(&workspace_id, &command_id)?;

        self.powershell.copy_to_clipboard(&command.program)
    }
}
