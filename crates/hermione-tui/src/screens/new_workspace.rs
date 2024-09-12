use crate::{
    clients::organizer::Client,
    models::new_workspace::Model,
    controllers::new_workspace::{Controller, ControllerParameters},
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct NewWorkspace<'a> {
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> NewWorkspace<'a> {
    pub fn enter(&mut self) -> Result<()> {
        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
            model: Model::new(),
        });

        if let Some(name) = controller.run()? {
            self.organizer.add_workspace(name)?;
        }

        Ok(())
    }
}
