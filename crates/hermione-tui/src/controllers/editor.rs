use crate::{models::editor::Message, Result};
use ratatui::{
    backend::Backend,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Frame, Terminal,
};

use super::event_handler;

pub trait FormModel: Sized {
    type Signal;

    fn view(&self, frame: &mut Frame);
    fn update(&mut self, message: Message) -> Result<Option<Self::Signal>>;
}

pub struct Controller<'a, B, F>
where
    B: Backend,
    F: FormModel,
{
    model: F,
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B, F>
where
    B: Backend,
    F: FormModel,
{
    pub model: F,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B, F> Controller<'a, B, F>
where
    B: Backend,
    F: FormModel,
{
    pub fn new(parameters: ControllerParameters<'a, B, F>) -> Self {
        let ControllerParameters { terminal, model } = parameters;

        Self { terminal, model }
    }

    pub fn run(mut self) -> Result<F::Signal> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            let maybe_message = event_handler(|event| Message::try_from(event).ok());

            if let Some(message) = maybe_message? {
                if let Some(signal) = self.model.update(message)? {
                    return Ok(signal);
                }
            }
        }
    }
}

impl TryFrom<KeyEvent> for Message {
    type Error = anyhow::Error;

    fn try_from(event: KeyEvent) -> std::result::Result<Self, Self::Error> {
        let message = match event.code {
            KeyCode::Left => Message::MoveCusorLeft,
            KeyCode::Right => Message::MoveCusorRight,
            KeyCode::Char(c) => Message::EnterChar(c),
            KeyCode::Enter => Message::Submit,
            KeyCode::Esc => Message::Exit,
            KeyCode::Tab => Message::Toggle,
            KeyCode::Backspace => match event.modifiers {
                KeyModifiers::CONTROL => Message::DeleteAllChars,
                _ => Message::DeleteChar,
            },
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported editor key code: {:?}",
                    event.code
                ))
            }
        };

        Ok(message)
    }
}
