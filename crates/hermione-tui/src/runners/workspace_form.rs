use std::{io::Stdout, time::Duration};

use anyhow::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};

use crate::models::{WorkspaceFormMessage, WorkspaceFormModel};

pub struct Runner<'a> {
    model: WorkspaceFormModel,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct RunnerParameters<'a> {
    pub model: WorkspaceFormModel,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Runner<'a> {
    pub fn new(parameters: RunnerParameters<'a>) -> Self {
        let RunnerParameters { model, terminal } = parameters;

        Self { model, terminal }
    }

    pub fn run(&mut self) -> Result<Option<String>> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message);
            }

            if self.model.is_exited() {
                break;
            }

            if self.model.is_submited() {
                return Ok(Some(self.model.name().into()));
            }
        }

        Ok(None)
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<WorkspaceFormMessage>> {
        let mut message = None;

        match key_code {
            KeyCode::Left => message = Some(WorkspaceFormMessage::MoveCusorLeft),
            KeyCode::Right => message = Some(WorkspaceFormMessage::MoveCusorRight),
            KeyCode::Char(c) => message = Some(WorkspaceFormMessage::EnterChar(c)),
            KeyCode::Backspace => message = Some(WorkspaceFormMessage::DeleteChar),
            KeyCode::Enter => message = Some(WorkspaceFormMessage::Submit),
            KeyCode::Esc => message = Some(WorkspaceFormMessage::Exit),
            _ => {}
        }

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<WorkspaceFormMessage>> {
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
