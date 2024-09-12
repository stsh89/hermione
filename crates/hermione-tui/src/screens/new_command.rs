use crate::{
    clients::organizer::{Client, CreateCommandParameters},
    controllers::new_command::{Controller, ControllerParameters, NewCommandParameters},
    models::new_command::Model,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct NewCommand<'a, B>
where
    B: Backend,
{
    pub workspace_id: usize,
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> NewCommand<'a, B>
where
    B: Backend,
{
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
