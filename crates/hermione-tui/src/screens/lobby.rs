use crate::{
    clients::organizer::Client,
    controllers::lobby::{Controller, ControllerParameters, Signal},
    entities::Workspace,
    models::lobby::{Model, ModelParameters},
    screens::{CommandCenter, NewWorkspace},
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
        if self.organizer.list_workspaces().is_empty() && self.new_workspace()?.is_none() {
            return Ok(());
        }

        loop {
            let signal = self.signal()?;
            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                model: Model::new(ModelParameters {
                    organizer: self.organizer,
                    session: self.session,
                })?,
                signal,
            });

            match controller.run()? {
                Signal::EnterCommandCenter(workspace_id) => {
                    self.enter_command_center(workspace_id)?
                }
                Signal::NewWorkspaceRequest => self.new_workspace().map(|_| ())?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn enter_command_center(&mut self, workspace_id: usize) -> Result<()> {
        self.session.set_workspace_id(Some(workspace_id))?;

        CommandCenter {
            workspace_id,
            organizer: self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }

    fn new_workspace(&mut self) -> Result<Option<Workspace>> {
        NewWorkspace {
            organizer: self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }

    fn signal(&mut self) -> Result<Option<Signal>> {
        if self.session.read_only() {
            if let Some(id) = self.session.get_workspace_id()? {
                return Ok(Some(Signal::EnterCommandCenter(id)));
            }
        }

        Ok(None)
    }
}
