use crate::{
    models::{TableauMessage, TableauModel},
    Result,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};
use std::{io::Stdout, time::Duration};

pub struct Runner<'a> {
    model: TableauModel<'a>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct RunnerParameters<'a> {
    pub model: TableauModel<'a>,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Runner<'a> {
    pub fn new(parameters: RunnerParameters<'a>) -> Self {
        let RunnerParameters { model, terminal } = parameters;

        Self { model, terminal }
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message)?;
            }

            if self.model.is_exited() {
                break;
            }
        }

        Ok(())
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<TableauMessage>> {
        let mut message = None;

        match key_code {
            KeyCode::Esc => message = Some(TableauMessage::Exit),
            KeyCode::Char('r') => message = Some(TableauMessage::RepeatCommand),
            _ => {}
        }

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<TableauMessage>> {
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
