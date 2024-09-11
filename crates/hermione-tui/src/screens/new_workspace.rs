use crate::{
    clients::OrganizerClient,
    models::new_workspace::Model,
    runners::new_workspace::{Runner, RunnerParameters},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct NewWorkspace<'a> {
    pub organizer: &'a mut OrganizerClient,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> NewWorkspace<'a> {
    pub fn enter(&mut self) -> Result<()> {
        let runner = Runner::new(RunnerParameters {
            terminal: self.terminal,
            model: Model::new(),
        });

        if let Some(name) = runner.run()? {
            self.organizer.add_workspace(name)?;
        }

        Ok(())
    }
}
