use super::handle_event;
use crate::{
    clients::organizer::Client,
    key_mappings::command_center_key_mapping,
    models::command_center::{Model, ModelParameters, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    organizer: &'a mut Client,
    workspace_number: usize,
    workspace_name: String,
    terminal: &'a mut Terminal<B>,
    location: String,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub workspace_number: usize,
    pub workspace_name: String,
    pub terminal: &'a mut Terminal<B>,
    pub location: String,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(params: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters {
            organizer,
            terminal,
            workspace_number,
            workspace_name,
            location,
        } = params;

        Self {
            organizer,
            workspace_number,
            workspace_name,
            terminal,
            location,
        }
    }

    pub fn run(self) -> Result<Signal> {
        let mut model = Model::new(ModelParameters {
            organizer: self.organizer,
            location: self.location,
            workspace_number: self.workspace_number,
            workspace_name: self.workspace_name,
        })?;

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event(command_center_key_mapping, model.input_mode())? {
                model = model.update(message)?;
            }
        }

        Ok(unsafe { model.signal() })
    }
}
