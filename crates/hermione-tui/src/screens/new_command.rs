use crate::{
    clients::organizer::{Client, CreateCommandParameters},
    models::new_command::Model,
    runners::new_command::{NewCommandParameters, Runner, RunnerParameters},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct NewCommand<'a> {
    pub workspace_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> NewCommand<'a> {
    pub fn enter(&mut self) -> Result<()> {
        let runner = Runner::new(RunnerParameters {
            terminal: self.terminal,
            model: Model::new(),
        });

        if let Some(new_command) = runner.run()? {
            let NewCommandParameters { name, program } = new_command;

            self.organizer.add_command(CreateCommandParameters {
                workspace_id: self.workspace_id,
                name,
                program,
            })?;
        }

        Ok(())
    }
}
