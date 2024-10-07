mod app_dir;
mod clients;
mod colors;
mod components;
mod handlers;
mod logs;
mod message;
mod models;
mod parameters;
mod presenters;
mod router;
mod routes;
mod tui;
mod widgets;

use app_dir::AppDir;
use clients::memories;
use router::Router;
use routes::Route;

pub use message::Message;

type Error = anyhow::Error;
type Model = dyn tui::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_dir = AppDir::new()?;

    tui::install_panic_hook();
    logs::init(app_dir.path())?;

    let memories = memories::Client::new(app_dir.path())?;

    tui::run(Router { memories })?;

    tui::restore_terminal()?;

    Ok(())
}