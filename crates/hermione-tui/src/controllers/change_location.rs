use std::env;

use super::handle_event;
use crate::{
    key_mappings::change_location_key_mapping,
    models::change_location::{Model, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters { terminal } = parameters;

        Self { terminal }
    }

    pub fn run(self) -> Result<Signal> {
        let mut model = Model::new(env::current_dir()?.display().to_string());

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event(change_location_key_mapping, model.input_mode())? {
                model = model.update(message);
            }
        }

        Ok(unsafe { model.signal() })
    }
}
