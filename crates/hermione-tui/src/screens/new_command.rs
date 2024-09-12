use crate::{
    clients::organizer::{Client, CreateCommandParameters},
    models::new_command::Model,
    controllers::new_command::{NewCommandParameters, Controller, ControllerParameters},
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
        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
            model: Model::new(),
        });

        if let Some(new_command) = controller.run()? {
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
