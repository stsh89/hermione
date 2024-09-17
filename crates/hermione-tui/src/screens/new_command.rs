use crate::{
    clients::organizer::{Client, CreateCommandParameters},
    controllers::new_command::{Controller, ControllerParameters},
    models::new_command::{NewCommandParameters, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct NewCommand<'a, B>
where
    B: Backend,
{
    pub workspace_number: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> NewCommand<'a, B>
where
    B: Backend,
{
    pub fn enter(&mut self) -> Result<()> {
        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
        });

        if let Signal::CreateNewCommand(new_command) = controller.run()? {
            let NewCommandParameters { name, program } = new_command;

            self.organizer.add_command(CreateCommandParameters {
                workspace_number: self.workspace_number,
                name,
                program,
            })?;
        }

        Ok(())
    }
}
