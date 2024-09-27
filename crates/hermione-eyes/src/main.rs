mod app;
mod clients;
mod helpers;
mod logs;
mod router;
mod routes;
mod settings;
mod tea;
mod tui;
mod types;

use app::{App, AppParameters};
use clients::memories;

fn main() -> types::Result<()> {
    let settings = settings::Settings::setup()?;

    tui::install_panic_hook();
    logs::init(settings.logs_path()?.as_str())?;

    let terminal = tui::init_terminal()?;
    let memories = memories::Client::new(settings.path_to_memories())?;
    let app = App::new(AppParameters { memories })?;

    app.run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
