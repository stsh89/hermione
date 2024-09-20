use super::handle_event;
use crate::{
    clients::organizer::Client,
    key_mappings::lobby_key_mapping,
    models::lobby::{Model, ModelParameters, Signal},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    organizer: &'a mut Client,
    terminal: &'a mut Terminal<B>,
    skip_lobby: bool,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
    pub skip_lobby: bool,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters {
            organizer,
            terminal,
            skip_lobby,
        } = parameters;

        Self {
            organizer,
            terminal,
            skip_lobby,
        }
    }

    pub fn run(self) -> Result<Signal> {
        if self.organizer.list_workspaces().is_empty() {
            return Ok(Signal::NewWorkspaceRequest);
        }

        if self.skip_lobby {
            return Ok(Signal::EnterCommandCenter(0));
        }

        let mut model = Model::new(ModelParameters {
            organizer: self.organizer,
        })?;

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event(lobby_key_mapping, model.input_mode())? {
                model = model.update(message)?;
            }
        }

        Ok(unsafe { model.signal() })
    }
}
