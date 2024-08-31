use crate::{desktop::Desktop, organizer::OrganizerCLient};

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let mut lens = Desktop::open(OrganizerCLient::initialize())?;

        loop {
            terminal.draw(|frame| lens.view(frame))?;

            let message = lens.handle_event()?;

            if let Some(message) = message {
                lens.update(message);
            }

            if lens.is_closed() {
                break;
            }
        }
        ratatui::restore();

        Ok(())
    }
}
