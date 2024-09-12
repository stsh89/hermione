use crate::{
    models::command_display::{Message, Model},
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

    pub fn run(mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message)?;
            }

            if self.model.is_exited() {
                return Ok(());
            }
        }
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<Message>> {
        let message = match key_code {
            KeyCode::Esc => Some(Message::Exit),
            KeyCode::Char('r') => Some(Message::RepeatCommand),
            _ => None,
        };

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
