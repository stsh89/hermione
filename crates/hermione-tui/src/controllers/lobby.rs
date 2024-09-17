use super::handle_event;
use crate::{
    clients::organizer::Client,
    key_mappings::lobby_key_mapping,
    models::lobby::{Model, ModelParameters, Signal},
    session::Session,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Controller<'a, B>
where
    B: Backend,
{
    organizer: &'a mut Client,
    session: &'a mut Session,
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub session: &'a mut Session,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters {
            organizer,
            session,
            terminal,
        } = parameters;

        Self {
            organizer,
            session,
            terminal,
        }
    }

    pub fn run(self) -> Result<Signal> {
        if self.organizer.list_workspaces().is_empty() {
            return Ok(Signal::NewWorkspaceRequest);
        }

        if self.session.read_only() {
            if let Some(workspace_number) = self.session.get_workspace_number()? {
                return Ok(Signal::EnterCommandCenter(workspace_number));
            }
        }

        let mut model = Model::new(ModelParameters {
            organizer: self.organizer,
            session: self.session,
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
