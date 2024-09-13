mod app;
mod clients;
mod controllers;
mod elements;
mod entities;
mod models;
mod screens;
mod session;

use anyhow::Result;
use app::App;
use clients::{organizer::Client as Organizer, session_loader::Client as SessionLoader};
use std::{io::Write, path::Path};

struct Connection {
    dir_path: String,
}

impl Connection {
    fn new() -> Result<Self> {
        let home_dir = match std::option_env!("HERMIONE_RELEASE") {
            Some(_) => dirs::home_dir()
                .ok_or(anyhow::anyhow!("can't get home dir"))?
                .to_str()
                .ok_or(anyhow::anyhow!("can't get home dir"))?
                .to_string(),
            None => std::env::var("CARGO_MANIFEST_DIR")?,
        };

        let dir_path = format!("{}/.hermione", home_dir);

        Ok(Self { dir_path })
    }

    fn initialize(&self) -> Result<()> {
        let path = Path::new(&self.dir_path);

        if !path.exists() {
            std::fs::create_dir(&self.dir_path)?;
        }

        self.initialize_session()?;
        self.initialize_organizer()?;

        Ok(())
    }

    fn initialize_session(&self) -> Result<()> {
        let session_path = self.session_path()?;
        let path = Path::new(&session_path);

        if path.exists() {
            return Ok(());
        }

        std::fs::File::create(path)?;
        let mut file = std::fs::File::create(path)?;
        file.write_all(b"{}")?;

        Ok(())
    }

    fn initialize_organizer(&self) -> Result<()> {
        let organizer_path = self.organizer_path()?;
        let path = Path::new(&organizer_path);

        if path.exists() {
            return Ok(());
        }

        let mut file = std::fs::File::create(path)?;
        file.write_all(b"[]")?;

        Ok(())
    }

    fn session_path(&self) -> Result<String> {
        let path = Path::new(&self.dir_path).join("session.json");

        Ok(path
            .to_str()
            .ok_or(anyhow::anyhow!("can't get organizer connection"))?
            .to_string())
    }

    fn organizer_path(&self) -> Result<String> {
        let path = Path::new(&self.dir_path).join("organizer.json");

        Ok(path
            .to_str()
            .ok_or(anyhow::anyhow!("can't get organizer connection"))?
            .to_string())
    }
}

fn main() -> Result<()> {
    tui::install_panic_hook();

    let connection = Connection::new()?;
    connection.initialize()?;

    let terminal = tui::init_terminal()?;
    let organizer = Organizer::new(connection.organizer_path()?)?;
    let session_loader = SessionLoader::new(connection.session_path()?);

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
