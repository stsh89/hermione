use crate::{
    models::new_workspace::{Message, Model},
    Result,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};
use std::io::Stdout;

pub struct Controller<'a> {
    model: Model,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct ControllerParameters<'a> {
    pub model: Model,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Controller<'a> {
    pub fn new(parameters: ControllerParameters<'a>) -> Self {
        let ControllerParameters { model, terminal } = parameters;

        Self { model, terminal }
    }

    pub fn run(mut self) -> Result<Option<String>> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message);
            }

            if self.model.is_exited() {
                return Ok(None);
            }

            if self.model.is_submited() {
                return Ok(Some(self.model.name().into()));
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
