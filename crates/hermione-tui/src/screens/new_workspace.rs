use crate::{
    clients::organizer::Client,
    controllers::new_workspace::{Controller, ControllerParameters},
    entities::Workspace,
    models::new_workspace::Model,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct NewWorkspace<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> NewWorkspace<'a, B>
where
    B: Backend,
{
    pub fn enter(&mut self) -> Result<Option<Workspace>> {
        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
            model: Model::new(),
        });

        if let Some(name) = controller.run()? {
            return Ok(Some(self.organizer.add_workspace(name)?));
        }

        Ok(None)
    }
}
