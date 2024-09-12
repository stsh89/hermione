use crate::{
    clients::organizer::Client,
    controllers::lobby::{Controller, ControllerParameters, Signal},
    models::lobby::{Model, ModelParameters},
    screens::{CommandCenter, NewWorkspace},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct Lobby<'a, B>
where
    B: Backend,
{
    pub organizer: &'a mut Client,
    pub terminal: &'a mut Terminal<B>,
}

impl<'a, B> Lobby<'a, B>
where
    B: Backend,
{
    pub fn enter(mut self) -> Result<()> {
        loop {
            let controller = Controller::new(ControllerParameters {
                terminal: self.terminal,
                model: Model::new(ModelParameters {
                    organizer: self.organizer,
                })?,
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
        CommandCenter {
            workspace_id,
            organizer: self.organizer,
            terminal: self.terminal,
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
