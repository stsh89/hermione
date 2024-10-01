use crate::{app::Handle, router::Router, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::{
    io::{stdout, Stdout},
    panic,
};

pub struct EventHandler<F, T>
where
    F: Fn(event::KeyEvent) -> Option<T>,
{
    f: F,
}

impl<F, T> EventHandler<F, T>
where
    F: Fn(event::KeyEvent) -> Option<T>,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }

    pub fn handle_event(self) -> Result<Option<T>> {
        let tui_event = event::read()?;

        if let event::Event::Key(key) = tui_event {
            if key.kind == event::KeyEventKind::Press {
                let message = (self.f)(key);

                return Ok(message);
            }
        }

        Ok(None)
    }
}

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

pub fn run(router: Router) -> Result<()> {
    tracing::info!("App started");

    let mut terminal = init_terminal()?;

    let Some(mut model) = router.handle_initial_route()? else {
        return Ok(());
    };

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

    tracing::info!("App stopped");

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
