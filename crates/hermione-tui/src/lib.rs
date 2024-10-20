mod app;
mod input;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::{
    io::{stdout, Stdout},
    panic,
};

pub use app::*;
pub use input::*;

type Result<T> = anyhow::Result<T>;

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(terminal)
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub fn run<R, M>(router: impl app::Router<Route = R, Message = M>) -> Result<()> {
    install_panic_hook();

    let mut terminal = init_terminal()?;
    let mut model = router.default_model()?;

    while model.is_running() {
        terminal.draw(|f| model.view(f))?;

        let mut maybe_message = model.handle_event()?;

        while let Some(message) = maybe_message {
            maybe_message = model.update(message)?;
        }

        if let Some(route) = model.redirect() {
            if let Some(change) = router.handle(route)? {
                model = change;
            }
        }
    }

    restore_terminal()
}
