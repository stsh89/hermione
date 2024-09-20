use std::env;

use crate::{
    clients::organizer::Client,
    controllers::lobby::{Controller, ControllerParameters},
    models::{lobby::Signal, workspace_editor::Signal as WorkspaceEditorSignal},
    screens::{CommandCenter, WorkspaceEditor},
    session::Session,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Lobby<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
    pub session: &'a mut Session,
}

impl<'a, B> Lobby<'a, B>
where
    B: Backend,
{
    pub fn enter(mut self) -> Result<()> {
        loop {
            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                organizer: self.organizer,
                session: self.session,
            });

            match controller.run()? {
                Signal::EnterCommandCenter(number) => self.enter_command_center(number)?,
                Signal::NewWorkspaceRequest => self.new_workspace()?,
                Signal::RenameWorkspace(number) => self.rename_workspace(number)?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn enter_command_center(&mut self, number: usize) -> Result<()> {
        self.organizer.promote_workspace(number)?;
        self.session.set_workspace_number(Some(0))?;

        CommandCenter {
            organizer: self.organizer,
            terminal: self.terminal,
            workspace_number: 0,
            location: env::current_dir()?.display().to_string(),
        }
        .enter()
    }

    fn new_workspace(&mut self) -> Result<()> {
        let signal = WorkspaceEditor {
            workspace: None,
            terminal: self.terminal,
        }
        .enter()?;

        if let WorkspaceEditorSignal::Submit(workspace_form) = signal {
            self.organizer.add_workspace(workspace_form.name)?;
        };

        Ok(())
    }

    fn rename_workspace(&mut self, number: usize) -> Result<()> {
        let workspace = self.organizer.get_workspace(number)?;

        let signal = WorkspaceEditor {
            workspace: Some(&workspace),
            terminal: self.terminal,
        }
        .enter()?;

        if let WorkspaceEditorSignal::Submit(workspace_form) = signal {
            self.organizer
                .rename_workspace(number, workspace_form.name)?;
        };

        Ok(())
    }
}
