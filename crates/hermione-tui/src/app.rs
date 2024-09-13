use crate::{
    clients::{organizer::Client as Organizer, session_loader::Client as SessionLoader},
    screens::Lobby,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App<B>
where
    B: Backend,
{
    pub organizer: Organizer,
    pub session_loader: SessionLoader,
    pub terminal: Terminal<B>,
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn run(mut self) -> Result<()> {
        let mut session = self.session_loader.load()?;

        let lobby = Lobby {
            organizer: &mut self.organizer,
            terminal: &mut self.terminal,
            session: &mut session,
        };

        lobby.enter()?;
        self.organizer.save()?;
        self.session_loader.save(session)?;

        Ok(())
    }
}
