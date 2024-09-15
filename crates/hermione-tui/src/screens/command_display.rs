use crate::{
    clients::organizer::Client,
    controllers::command_display::{Controller, ControllerParameters},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct CommandDisplay<'a, B>
where
    B: Backend,
{
    pub workspace_id: usize,
    pub command_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> CommandDisplay<'a, B>
where
    B: Backend,
{
    pub fn enter(&mut self) -> Result<()> {
        let command = self
            .organizer
            .get_command(self.workspace_id, self.command_id)?;

        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
            command,
        });

        controller.run()?;

        Ok(())
    }
}
