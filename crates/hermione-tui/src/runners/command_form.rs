use std::{io::Stdout, time::Duration};

use anyhow::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};

use crate::models::{CommandFormMessage, CommandFormModel, NewCommand};

pub struct Runner<'a> {
    model: CommandFormModel,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct RunnerParameters<'a> {
    pub model: CommandFormModel,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Runner<'a> {
    pub fn new(parameters: RunnerParameters<'a>) -> Self {
        let RunnerParameters { model, terminal } = parameters;

        Self { model, terminal }
    }

    pub fn run(&mut self) -> Result<Option<NewCommand>> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message);
            }

            if self.model.is_exited() {
                return Ok(None);
            }

            if self.model.is_submited() {
                return Ok(Some(self.model.new_command()));
            }
        }
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<CommandFormMessage>> {
        let mut message = None;

        match key_code {
            KeyCode::Left => message = Some(CommandFormMessage::MoveCusorLeft),
            KeyCode::Right => message = Some(CommandFormMessage::MoveCusorRight),
            KeyCode::Char(c) => message = Some(CommandFormMessage::EnterChar(c)),
            KeyCode::Backspace => message = Some(CommandFormMessage::DeleteChar),
            KeyCode::Enter => message = Some(CommandFormMessage::Submit),
            KeyCode::Esc => message = Some(CommandFormMessage::Exit),
            KeyCode::Tab => message = Some(CommandFormMessage::ToggleActiveInput),
            _ => {}
        }

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<CommandFormMessage>> {
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    let message = self.handle_key(key.code)?;

                    return Ok(message);
                }
            }
        }

        Ok(None)
    }
}
