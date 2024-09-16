use crate::{
    clients::organizer::Client,
    controllers::lobby::{Controller, ControllerParameters},
    models::lobby::Signal,
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
        loop {
            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                organizer: self.organizer,
                session: self.session,
            });

            match controller.run()? {
                Signal::EnterCommandCenter(workspace_id) => {
                    self.enter_command_center(workspace_id)?
                }
                Signal::NewWorkspaceRequest => self.new_workspace()?,
                Signal::Exit => break,
            };
        }

        Ok(())
    }

    fn enter_command_center(&mut self, workspace_id: usize) -> Result<()> {
        self.organizer.promote_workspace(workspace_id)?;
        self.session.set_workspace_id(Some(0))?;

        CommandCenter {
            organizer: self.organizer,
            terminal: self.terminal,
            workspace_id: 0,
        }
        .enter()
    }

    fn new_workspace(&mut self) -> Result<()> {
        NewWorkspace {
            organizer: self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }
}
