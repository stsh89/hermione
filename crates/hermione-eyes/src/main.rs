mod app;
mod entities;
mod models;
mod router;
mod tui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    tui::install_panic_hook();

    let terminal = tui::init_terminal()?;
    let app = App::new();

    app.run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
