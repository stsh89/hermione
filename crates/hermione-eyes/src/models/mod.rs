mod get_command;
mod get_workspace;
mod helpers;
mod list_workspaces;
mod new_command;
mod new_workspace;

use crate::{router::Router, Result};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers},
    style::{Style, Stylize},
    Frame,
};

pub use get_command::{GetCommandModel, GetCommandModelParameters};
pub use get_workspace::{GetWorkspaceModel, GetWorkspaceModelParameters};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
pub use new_command::NewCommandModel;
pub use new_workspace::NewWorkspaceModel;

pub trait Model {
    fn handle_event(&self) -> Result<Option<Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        true
    }

    fn redirect(&self) -> Option<&Router> {
        None
    }

    fn update(&mut self, _message: Message) -> Result<Option<Message>> {
        Ok(None)
    }

    fn view(&mut self, _frame: &mut Frame) {}
}

pub enum Message {
    ToggleCommandPalette,
    Back,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    MoveCusorLeft,
    MoveCusorRight,
    SelectNext,
    SelectPrevious,
    Submit,
    ToggleFocus,
}

struct EventHandler<F>
where
    F: Fn(event::KeyEvent) -> Option<Message>,
{
    f: F,
}

impl<F> EventHandler<F>
where
    F: Fn(event::KeyEvent) -> Option<Message>,
{
    fn new(f: F) -> Self {
        Self { f }
    }

    fn handle_event(self) -> Result<Option<Message>> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
}

impl TryFrom<KeyEvent> for Message {
    type Error = anyhow::Error;

    fn try_from(key_event: KeyEvent) -> Result<Self> {
        let message = match key_event.code {
            KeyCode::Tab => Message::ToggleFocus,
            KeyCode::Up => Message::SelectPrevious,
            KeyCode::Down => Message::SelectNext,
            KeyCode::Esc => Message::Back,
            KeyCode::Enter => Message::Submit,
            KeyCode::Left => Message::MoveCusorLeft,
            KeyCode::Right => Message::MoveCusorRight,
            KeyCode::Backspace => match key_event.modifiers {
                KeyModifiers::CONTROL => Message::DeleteAllChars,
                _ => Message::DeleteChar,
            },
            KeyCode::Char(c) => match key_event.modifiers {
                KeyModifiers::CONTROL => match c {
                    'k' => Message::ToggleCommandPalette,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Unsupported key code: {:?}",
                            key_event.code
                        ))
                    }
                },
                _ => Message::EnterChar(c),
            },
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported key code: {:?}",
                    key_event.code
                ))
            }
        };

        Ok(message)
    }
}

fn highlight_style() -> Style {
    Style::default().on_light_blue()
}
