use std::env;

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
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> ChangeLocation<'a, B>
where
    B: Backend,
{
    pub fn enter(&mut self) -> Result<()> {
        let controller = Controller::new(ControllerParameters {
            terminal: self.terminal,
        });

        if let Signal::ChangeLocation(name) = controller.run()? {
            env::set_current_dir(&name)?;
        }

        Ok(())
    }
}
