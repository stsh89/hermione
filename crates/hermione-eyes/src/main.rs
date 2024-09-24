mod app;
mod clients;
mod entities;
mod models;
mod router;
mod settings;
mod tui;

use anyhow::Result;
use app::{App, AppParameters};
use settings::Settings;

fn main() -> Result<()> {
    tui::install_panic_hook();

    let terminal = tui::init_terminal()?;
    let settings = Settings::setup()?;
    let organizer = clients::organizer::Client::new(settings.organizer_path()?)?;

    let app = App::new(AppParameters { organizer })?;

    app.run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
