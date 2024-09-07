use crate::{
    clients::OrganizerClient,
    models::{ShowcaseModel, ShowcaseModelParameters},
    runners::{ShowcaseRunner, ShowcaseRunnerParameters},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct App {
    pub organizer: OrganizerClient,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn run(mut self) -> Result<()> {
        let runner = ShowcaseRunner::new(ShowcaseRunnerParameters {
            terminal: &mut self.terminal,
            model: ShowcaseModel::new(ShowcaseModelParameters {
                organizer: &mut self.organizer,
            }),
        });

        runner.run()?;

        Ok(())
    }
}
