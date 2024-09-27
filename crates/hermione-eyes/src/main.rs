mod app;
mod clients;
mod entities;
mod logs;
mod models;
mod router;
mod settings;
mod tui;

use anyhow::Result;
use app::{App, AppParameters};
use clients::memories;
use settings::Settings;

fn main() -> Result<()> {
    let settings = Settings::setup()?;

    tui::install_panic_hook();
    logs::init(settings.logs_path()?.as_str())?;

    let terminal = tui::init_terminal()?;
    let memories = memories::Client::new(memories::json::Client::new(
        settings.path_to_memories().clone(),
    )?);
    let app = App::new(AppParameters { memories })?;

    app.run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
