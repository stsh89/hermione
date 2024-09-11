use crate::{
    clients::organizer::Client,
    models::command_display::{Model, ModelParameters},
    runners::command_display::{Runner, RunnerParameters},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct CommandDisplay<'a> {
    pub workspace_id: usize,
    pub command_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> CommandDisplay<'a> {
    pub fn enter(&mut self) -> Result<()> {
        let command = self
            .organizer
            .get_command(self.workspace_id, self.command_id)?;
        let runner = Runner::new(RunnerParameters {
            terminal: self.terminal,
            model: Model::new(ModelParameters { command })?,
        });

        runner.run()
    }
}
