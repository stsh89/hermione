use super::handle_event;
use crate::{
    entities::Command,
    key_mappings::command_display_key_mapping,
    models::command_display::{Model, ModelParameters, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    command: Command,
    location: String,
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub command: Command,
    pub location: String,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters {
            command,
            terminal,
            location,
        } = parameters;

        Self {
            command,
            terminal,
            location,
        }
    }

    pub fn run(self) -> Result<Signal> {
        let mut model = Model::new(ModelParameters {
            command: self.command,
            location: self.location,
        })?;

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event(command_display_key_mapping, model.input_mode())? {
                model = model.update(message)?;
            }
        }

        Ok(unsafe { model.signal() })
    }
}
