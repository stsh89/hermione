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
                Signal::EnterCommandCenter(number) => self.enter_command_center(number)?,
                Signal::NewWorkspaceRequest => self.new_workspace()?,
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
