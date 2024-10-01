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

use clients::memories;
use router::Router;
use routes::Route;

type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let settings = settings::Settings::setup()?;

    tui::install_panic_hook();
    logs::init(settings.logs_path()?.as_str())?;

    let terminal = tui::init_terminal()?;
    let memories = memories::Client::new(settings.path_to_memories())?;

    app::run(app::Parameters {
        terminal,
        router: Router { memories },
        route: Route::Workspaces(routes::workspaces::Route::New),
    })?;

    tui::restore_terminal()?;

    Ok(())
}
