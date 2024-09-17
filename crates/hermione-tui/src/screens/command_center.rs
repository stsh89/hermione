use crate::{
    clients::organizer::Client,
    controllers::command_center::{Controller, ControllerParameters},
    models::command_center::Signal,
    screens::{ChangeLocation, CommandDisplay, NewCommand},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct CommandCenter<'a, B>
where
    B: Backend,
{
    pub workspace_number: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> CommandCenter<'a, B>
where
    B: Backend,
{
    fn execute_command(&mut self, command_number: usize) -> Result<()> {
        self.organizer
            .promote_command(self.workspace_number, command_number)?;

        CommandDisplay {
            command_number: 0,
            organizer: self.organizer,
            terminal: self.terminal,
            workspace_number: self.workspace_number,
        }
        .enter()
    }

    pub fn enter(mut self) -> Result<()> {
        loop {
            let workspace = self.organizer.get_workspace(self.workspace_number)?;

            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                organizer: self.organizer,
                workspace_name: workspace.name,
                workspace_number: workspace.number,
            });

            match controller.run()? {
                Signal::ExecuteCommand(command_number) => self.execute_command(command_number)?,
                Signal::NewCommandRequest => self.new_command()?,
                Signal::ChangeLocationRequest => self.change_location()?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn new_command(&mut self) -> Result<()> {
        NewCommand {
            terminal: self.terminal,
            workspace_number: self.workspace_number,
            organizer: self.organizer,
        }
        .enter()
    }

    fn change_location(&mut self) -> Result<()> {
        ChangeLocation {
            terminal: self.terminal,
        }
        .enter()
    }
}
