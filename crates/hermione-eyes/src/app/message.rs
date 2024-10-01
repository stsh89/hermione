use crate::{routes::Router, Error, Result};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers},
    Frame,
};

pub trait Hook {
    fn handle_event(&self) -> Result<Option<Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        true
    }

    fn redirect(&mut self) -> Option<Router> {
        None
    }

    fn update(&mut self, _message: Message) -> Result<Option<Message>> {
        Ok(None)
    }

    fn view(&mut self, _frame: &mut Frame) {}
}

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
        let tui_event = event::read()?;

        if let event::Event::Key(key) = tui_event {
            if key.kind == event::KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
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
