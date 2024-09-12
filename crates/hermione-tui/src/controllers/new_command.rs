use crate::{
    models::new_command::{Message, Model},
    Result,
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
    model: Model,
    terminal: &'a mut Terminal<B>,
}

pub struct ControllerParameters<'a, B>
where
    B: Backend,
{
    pub model: Model,
    pub terminal: &'a mut Terminal<B>,
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
}

impl<'a, B> Controller<'a, B>
where
    B: Backend,
{
    pub fn new(parameters: ControllerParameters<'a, B>) -> Self {
        let ControllerParameters { model, terminal } = parameters;

        Self { model, terminal }
    }

    pub fn run(mut self) -> Result<Option<NewCommandParameters>> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message);
            }

            if self.model.is_exited() {
                return Ok(None);
            }

            if self.model.is_submited() {
                return Ok(Some(NewCommandParameters {
                    name: self.model.name().into(),
                    program: self.model.program().into(),
                }));
            }
        }
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<Message>> {
        let mut message = None;

        match key_code {
            KeyCode::Left => message = Some(Message::MoveCusorLeft),
            KeyCode::Right => message = Some(Message::MoveCusorRight),
            KeyCode::Char(c) => message = Some(Message::EnterChar(c)),
            KeyCode::Backspace => message = Some(Message::DeleteChar),
            KeyCode::Enter => message = Some(Message::Submit),
            KeyCode::Esc => message = Some(Message::Exit),
            KeyCode::Tab => message = Some(Message::ToggleFormInput),
            _ => {}
        }

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<Message>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let message = self.handle_key(key.code)?;

                return Ok(message);
            }
        }

        Ok(None)
    }
}
