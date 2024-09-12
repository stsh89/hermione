mod clients;
mod controllers;
mod data;
mod elements;
mod models;
mod screens;

use anyhow::Result;
use clients::organizer::Client;

fn main() -> Result<()> {
    tui::install_panic_hook();

    let mut terminal = tui::init_terminal()?;
    let mut organizer = Client::new()?;
    let lobby = screens::Lobby {
        organizer: &mut organizer,
        terminal: &mut terminal,
    };

    lobby.enter()?;
    tui::restore_terminal()?;

    Ok(())
}

mod tui {
    use super::Result;
    use ratatui::{
        backend::CrosstermBackend,
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };
    use std::{
        io::{stdout, Stdout},
        panic,
    };

    pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook() {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}