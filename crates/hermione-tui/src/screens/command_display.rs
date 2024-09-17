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
    pub workspace_number: usize,
    pub command_number: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
    pub location: String,
}

impl<'a, B> CommandDisplay<'a, B>
where
    B: Backend,
{
    pub fn enter(self) -> Result<()> {
        let command = self
            .organizer
            .get_command(self.workspace_number, self.command_number)?;

        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
            location: self.location,
            command,
        });

        controller.run()?;

        Ok(())
    }
}
