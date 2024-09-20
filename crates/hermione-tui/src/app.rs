use crate::{clients::organizer::Client as Organizer, screens::Lobby, Result};
use ratatui::{backend::Backend, Terminal};

pub struct App<B>
where
    B: Backend,
{
    pub organizer: Organizer,
    pub terminal: Terminal<B>,
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn run(mut self) -> Result<()> {
        let lobby = Lobby {
            organizer: &mut self.organizer,
            terminal: &mut self.terminal,
        };

        lobby.enter()?;
        self.organizer.save()?;

        Ok(())
    }
}
