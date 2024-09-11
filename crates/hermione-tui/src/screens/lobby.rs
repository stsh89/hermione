use crate::{
    clients::organizer::Client,
    models::lobby::{Model, ModelParameters},
    runners::lobby::{Runner, RunnerParameters, Signal},
    screens::{CommandCenter, NewWorkspace},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct Lobby<'a> {
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Lobby<'a> {
    pub fn enter(mut self) -> Result<()> {
        loop {
            let runner = Runner::new(RunnerParameters {
                terminal: self.terminal,
                model: Model::new(ModelParameters {
                    organizer: self.organizer,
                })?,
            });

            match runner.run()? {
                Signal::EnterCommandCenter(workspace_id) => {
                    self.enter_command_center(workspace_id)?
                }
                Signal::NewWorkspaceRequest => self.new_workspace()?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn enter_command_center(&mut self, workspace_id: usize) -> Result<()> {
        CommandCenter {
            workspace_id,
            organizer: self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }

    fn new_workspace(&mut self) -> Result<()> {
        NewWorkspace {
            organizer: self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }
}
