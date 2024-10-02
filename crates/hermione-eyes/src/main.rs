mod clients;
mod components;
mod controllers;
mod logs;
mod message;
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
use routes::Route;

pub use message::Message;

type Error = anyhow::Error;
type Model = dyn tui::Model<Route = Route, Message = Message>;
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
