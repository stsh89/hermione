mod app;
mod clients;
mod controllers;
mod helpers;
mod logs;
mod models;
mod parameters;
mod presenters;
mod router;
mod routes;
mod settings;
mod tui;
mod widgets;

use clients::memories;
use router::Router;

type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let settings = settings::Settings::setup()?;

    tui::install_panic_hook();
    logs::init(settings.logs_path()?.as_str())?;

    let memories = memories::Client::new(settings.path_to_memories())?;

    tui::run(Router { memories })?;

    tui::restore_terminal()?;

    Ok(())
}
