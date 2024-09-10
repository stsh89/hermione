use crate::{
    models::{CommandCenterMessage, CommandCenterModel, CommandFormModel},
    Result,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};
use std::io::Stdout;

use super::{
    CommandFormRunner, CommandFormRunnerParameters, TableauRunner, TableauRunnerParameters,
};

pub struct Runner<'a> {
    model: CommandCenterModel<'a>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct RunnerParameters<'a> {
    pub model: CommandCenterModel<'a>,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Runner<'a> {
    pub fn new(params: RunnerParameters<'a>) -> Self {
        let RunnerParameters { model, terminal } = params;

        Self { model, terminal }
    }

    pub fn run(&mut self) -> Result<Option<String>> {
        loop {
            self.terminal.draw(|frame| self.model.view(frame))?;

            if let Some(message) = self.handle_event()? {
                self.model.update(message)?;
            }

            if self.model.is_exited() {
                break;
            }
        }

        Ok(None)
    }

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<CommandCenterMessage>> {
        let message = match key_code {
            KeyCode::Up => Some(CommandCenterMessage::SelectPreviousCommand),
            KeyCode::Down => Some(CommandCenterMessage::SelectNextCommand),
            KeyCode::Esc => Some(CommandCenterMessage::Exit),
            KeyCode::Char(c) if self.model.in_editor_mode() => {
                Some(CommandCenterMessage::EnterChar(c))
            }
            KeyCode::Left if self.model.in_editor_mode() => {
                Some(CommandCenterMessage::MoveCusorLeft)
            }
            KeyCode::Right if self.model.in_editor_mode() => {
                Some(CommandCenterMessage::MoveCusorRight)
            }
            KeyCode::Backspace if self.model.in_editor_mode() => {
                Some(CommandCenterMessage::DeleteChar)
            }
            KeyCode::Char('n') if self.model.in_normal_mode() => self.new_command()?,
            KeyCode::Char('d') if self.model.in_normal_mode() => {
                Some(CommandCenterMessage::DeleteCommand)
            }
            KeyCode::Enter if self.model.has_selected_command() => {
                self.execute_command().map(|_| None)?
            }
            KeyCode::Char('s') if self.model.in_normal_mode() => {
                Some(CommandCenterMessage::ActivateSearchBar)
            }
            _ => None,
        };

        Ok(message)
    }

    fn execute_command(&mut self) -> Result<()> {
        let Some(model) = self.model.tableau()? else {
            return Ok(());
        };

        let runner = TableauRunner::new(TableauRunnerParameters {
            model,
            terminal: self.terminal,
        });

        runner.run()?;

        Ok(())
    }

    fn new_command(&mut self) -> Result<Option<CommandCenterMessage>> {
        let mut runner = CommandFormRunner::new(CommandFormRunnerParameters {
            model: CommandFormModel::new(),
            terminal: self.terminal,
        });

        let message = runner.run()?.map(CommandCenterMessage::CreateCommand);

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<CommandCenterMessage>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                let message = self.handle_key(key.code)?;

                return Ok(message);
            }
        }

        Ok(None)
    }
}
