use crate::{
    clients::organizer::Client,
    controllers::event_handler,
    models::command_center::{Message, Model, ModelParameters, Signal},
    Result,
};
use ratatui::{
    backend::Backend,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Terminal,
};

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

        loop {
            self.terminal.draw(|frame| model.view(frame))?;

            let maybe_message = event_handler(|event| from_event(event, &model));

            if let Some(message) = maybe_message? {
                if let Some(signal) = model.update(message)? {
                    return Ok(signal);
                }
            }
        }
    }
}

pub fn from_event(event: KeyEvent, model: &Model) -> Option<Message> {
    let message = match event.code {
        KeyCode::Char(c) => {
            if model.is_editing() {
                Message::EnterChar(c)
            } else {
                match c {
                    'c' => Message::ChangeLocationRequest,
                    'd' => Message::DeleteCommand,
                    'e' => Message::EditCommand,
                    'n' => Message::NewCommandRequest,
                    's' => Message::ActivateSearchBar,
                    _ => return None,
                }
            }
        }
        KeyCode::Left if model.is_editing() => Message::MoveCusorLeft,
        KeyCode::Right if model.is_editing() => Message::MoveCusorRight,
        KeyCode::Up => Message::SelectPreviousCommand,
        KeyCode::Down => Message::SelectNextCommand,
        KeyCode::Esc => Message::Exit,
        KeyCode::Enter => match event.modifiers {
            KeyModifiers::CONTROL => Message::RunCommand,
            _ => Message::ExecuteCommand,
        },
        KeyCode::Backspace => match event.modifiers {
            KeyModifiers::CONTROL => Message::DeleteAllChars,
            _ => Message::DeleteChar,
        },
        _ => return None,
    };

    Some(message)
}
