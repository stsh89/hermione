mod app;
mod clients;
mod entities;
mod logs;
mod models;
mod router;
mod settings;
mod tui;

use anyhow::{Error, Result};
use app::{App, AppParameters};
use settings::Settings;

fn main() -> Result<()> {
    let settings = Settings::setup()?;

    tui::install_panic_hook();
    logs::init(settings.logs_path()?.as_str())?;

    let terminal = tui::init_terminal()?;
    let organizer = clients::organizer::Client::new(settings.organizer_path()?);
    let app = App::new(AppParameters { organizer })?;

    app.run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
