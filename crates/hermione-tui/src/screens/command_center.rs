use crate::{
    clients::organizer::Client,
    models::command_center::{Model, ModelParameters},
    runners::command_center::{Runner, RunnerParameters, Signal},
    screens::{CommandDisplay, NewCommand},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct CommandCenter<'a> {
    pub workspace_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> CommandCenter<'a> {
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

            let runner = Runner::new(RunnerParameters {
                terminal: self.terminal,
                model: Model::new(ModelParameters {
                    organizer: self.organizer,
                    workspace,
                })?,
            });

            match runner.run()? {
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
