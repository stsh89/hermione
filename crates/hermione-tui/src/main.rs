mod app;
mod clients;
mod controllers;
mod elements;
mod entities;
mod environment;
mod models;
mod screens;
mod session;

use anyhow::Result;
use app::App;
use clients::{organizer::Client as Organizer, session_loader::Client as SessionLoader};
use environment::Environment;

fn main() -> Result<()> {
    tui::install_panic_hook();

    let environment = Environment::setup()?;
    let terminal = tui::init_terminal()?;
    let organizer = Organizer::new(environment.organizer_path()?)?;
    let session_loader = SessionLoader::new(environment.session_path()?);

    let app = App {
        organizer,
        session_loader,
        terminal,
    };

    app.run()?;
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
