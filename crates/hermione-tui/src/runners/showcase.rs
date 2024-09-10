use crate::{
    models::{ShowcaseMessage, ShowcaseModel, WorkspaceFormModel},
    Result,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::CrosstermBackend,
    Terminal,
};
use std::{io::Stdout, time::Duration};

use super::{
    CommandCenterRunner, CommandCenterRunnerParameters, WorkspaceFormRunner,
    WorkspaceFormRunnerParameters,
};

pub struct Runner<'a> {
    model: ShowcaseModel<'a>,
    terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

pub struct RunnerParameters<'a> {
    pub model: ShowcaseModel<'a>,
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

    pub fn handle_key(&mut self, key_code: KeyCode) -> Result<Option<ShowcaseMessage>> {
        let mut message = None;

        match key_code {
            KeyCode::Up => message = Some(ShowcaseMessage::SelectPreviousWorkspace),
            KeyCode::Down => message = Some(ShowcaseMessage::SelectNextWorkspace),
            KeyCode::Char('n') => message = self.maybe_create_workspace_message()?,
            KeyCode::Char('d') => message = Some(ShowcaseMessage::DeleteWorkspace),
            KeyCode::Esc => message = Some(ShowcaseMessage::Exit),
            KeyCode::Enter => self.run_command_center()?,
            KeyCode::Char('q') => message = Some(ShowcaseMessage::Exit),
            KeyCode::Char('s') => message = Some(ShowcaseMessage::Save),
            _ => {}
        };

        Ok(message)
    }

    fn run_command_center(&mut self) -> Result<()> {
        let Some(model) = self.model.command_center()? else {
            return Ok(());
        };

        let mut command_center = CommandCenterRunner::new(CommandCenterRunnerParameters {
            model,
            terminal: self.terminal,
        });

        command_center.run()?;
        self.model.reset_selector();

        Ok(())
    }

    fn maybe_create_workspace_message(&mut self) -> Result<Option<ShowcaseMessage>> {
        let mut runner = WorkspaceFormRunner::new(WorkspaceFormRunnerParameters {
            model: WorkspaceFormModel::new(),
            terminal: self.terminal,
        });

        let message = runner.run()?.map(ShowcaseMessage::CreateWorkspace);

        Ok(message)
    }

    fn handle_event(&mut self) -> Result<Option<ShowcaseMessage>> {
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
