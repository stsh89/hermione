use crate::types::{Error, Result};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Message {
    Back,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    MoveCusorLeft,
    MoveCusorRight,
    SelectNext,
    SelectPrevious,
    Submit,
    ToggleCommandPalette,
    ToggleFocus,
    ExecuteCommand,
}

impl TryFrom<KeyEvent> for Message {
    type Error = Error;

    fn try_from(key_event: KeyEvent) -> Result<Self> {
        let message = match key_event.code {
            KeyCode::Tab => Message::ToggleFocus,
            KeyCode::Up => Message::SelectPrevious,
            KeyCode::Down => Message::SelectNext,
            KeyCode::Esc => Message::Back,
            KeyCode::Enter => match key_event.modifiers {
                KeyModifiers::CONTROL => Message::ExecuteCommand,
                _ => Message::Submit,
            },
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
