use crate::{
    controllers::change_location::{Controller, ControllerParameters},
    models::change_location::Signal,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct ChangeLocation<'a, B>
where
    B: Backend,
{
    pub location: String,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> ChangeLocation<'a, B>
where
    B: Backend,
{
    pub fn enter(self) -> Result<Signal> {
        let controller = Controller::new(ControllerParameters {
            location: self.location,
            terminal: self.terminal,
        });

        controller.run()
    }
}
