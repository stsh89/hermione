use crate::{
    clients::organizer::Client, models::lobby::{Message, Model, ModelParameters, Signal}, session::Session, Result
};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    Terminal,
};

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
            if let Some(workspace_id) = self.session.get_workspace_id()? {
                return Ok(Signal::EnterCommandCenter(workspace_id));
            }
        }

        let mut model = Model::new(ModelParameters {
            organizer: self.organizer,
            session: self.session,
        })?;

        while model.is_running() {
            self.terminal.draw(|frame| model.view(frame))?;

            if let Some(message) = handle_event()? {
                model = model.update(message)?;
            }
        }

        Ok(unsafe { model.signal() })
    }
}

fn handle_event() -> Result<Option<Message>> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = handle_key(key.code)?;

            return Ok(message);
        }
    }

    Ok(None)
}

fn handle_key(key_code: KeyCode) -> Result<Option<Message>> {
    let message = match key_code {
        KeyCode::Up => Some(Message::SelectPreviousWorkspace),
        KeyCode::Down => Some(Message::SelectNextWorkspace),
        KeyCode::Char('n') => Some(Message::NewWorkspaceRequest),
        KeyCode::Char('d') => Some(Message::DeleteWorkspace),
        KeyCode::Esc => Some(Message::Exit),
        KeyCode::Enter => Some(Message::EnterCommandCenter),
        KeyCode::Char('q') => Some(Message::Exit),
        _ => None,
    };

    Ok(message)
}
