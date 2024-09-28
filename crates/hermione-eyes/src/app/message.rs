use crate::types::{Error, Result};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Message {
    Action,
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
}

impl TryFrom<KeyEvent> for Message {
    type Error = Error;

    fn try_from(key_event: KeyEvent) -> Result<Self> {
        let message = match key_event.code {
            KeyCode::Tab => Self::ToggleFocus,
            KeyCode::Up => Self::SelectPrevious,
            KeyCode::Down => Self::SelectNext,
            KeyCode::Esc => Self::Back,
            KeyCode::Enter => match key_event.modifiers {
                KeyModifiers::CONTROL => Self::Action,
                _ => Self::Submit,
            },
            KeyCode::F(1) => Self::Action,
            KeyCode::Left => Self::MoveCusorLeft,
            KeyCode::Right => Self::MoveCusorRight,
            KeyCode::Backspace => match key_event.modifiers {
                KeyModifiers::CONTROL => Self::DeleteAllChars,
                _ => Self::DeleteChar,
            },
            KeyCode::Char(c) => match key_event.modifiers {
                KeyModifiers::CONTROL => match c {
                    'k' => Self::ToggleCommandPalette,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Unsupported key code: {:?}",
                            key_event.code
                        ))
                    }
                },
                _ => Self::EnterChar(c),
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
