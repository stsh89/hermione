use crate::{Error, Result};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Message {
    Cancel,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    ExecuteCommand,
    MoveCusorLeft,
    MoveCusorRight,
    SelectNext,
    SelectPrevious,
    Submit,
    Tab,
}

impl TryFrom<KeyEvent> for Message {
    type Error = Error;

    fn try_from(key_event: KeyEvent) -> Result<Self> {
        let message = match key_event.code {
            KeyCode::Tab => Self::Tab,
            KeyCode::Up => Self::SelectPrevious,
            KeyCode::Down => Self::SelectNext,
            KeyCode::Esc => Self::Cancel,
            KeyCode::Enter => match key_event.modifiers {
                KeyModifiers::CONTROL => Self::ExecuteCommand,
                _ => Self::Submit,
            },
            KeyCode::F(1) => Self::ExecuteCommand,
            KeyCode::Left => Self::MoveCusorLeft,
            KeyCode::Right => Self::MoveCusorRight,
            KeyCode::Backspace => match key_event.modifiers {
                KeyModifiers::CONTROL => Self::DeleteAllChars,
                _ => Self::DeleteChar,
            },
            KeyCode::Char(c) => Self::EnterChar(c),
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
