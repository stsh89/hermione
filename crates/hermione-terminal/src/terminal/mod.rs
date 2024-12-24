mod ui;

use crate::program_lib::{Render, State};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
};
use std::{
    io::{stdout, Stdout},
    panic,
};

pub struct Terminal {
    inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Render for Terminal {
    fn render(&mut self, state: &State) -> anyhow::Result<()> {
        self.inner.draw(|frame| ui::render(frame, state))?;

        Ok(())
    }
}

pub fn init() -> anyhow::Result<Terminal> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let inner = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;

    Ok(Terminal { inner })
}

pub fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        stdout().execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

pub fn restore() -> anyhow::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
