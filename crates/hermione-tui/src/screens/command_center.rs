use crate::{
    clients::organizer::Client,
    controllers::command_center::{Controller, ControllerParameters, Signal},
    models::command_center::{Model, ModelParameters},
    screens::{CommandDisplay, NewCommand},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct CommandCenter<'a, B>
where
    B: Backend,
{
    pub workspace_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> CommandCenter<'a, B>
where
    B: Backend,
{
    fn execute_command(&mut self, command_id: usize) -> Result<()> {
        CommandDisplay {
            organizer: self.organizer,
            terminal: self.terminal,
            command_id,
            workspace_id: self.workspace_id,
        }
        .enter()
    }

    pub fn enter(mut self) -> Result<()> {
        loop {
            let workspace = self.organizer.get_workspace(self.workspace_id)?;

            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                model: Model::new(ModelParameters {
                    organizer: self.organizer,
                    workspace_name: workspace.name,
                    workspace_id: workspace.id,
                })?,
            });

            match controller.run()? {
                Signal::ExecuteCommand(command_id) => self.execute_command(command_id)?,
                Signal::NewCommandRequest => self.new_command()?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn new_command(&mut self) -> Result<()> {
        NewCommand {
            terminal: self.terminal,
            workspace_id: self.workspace_id,
            organizer: self.organizer,
        }
        .enter()
    }
}
