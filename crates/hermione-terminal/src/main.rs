mod app_dir;
// mod clients;
mod colors;
mod components;
mod handlers;
mod integrations;
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

use app_dir::AppDir;
use router::Router;
use routes::Route;

pub use message::Message;
use settings::Settings;

type Error = anyhow::Error;
type Model = dyn tui::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_dir = AppDir::new()?;

    tui::install_panic_hook();
    logs::init(app_dir.path())?;

    let settings = Settings::new(app_dir.path())?;

    let workspaces = match settings.storage {
        settings::Storage::Json => integrations::core::workspaces::Client::json(app_dir.path())?,
        settings::Storage::Notion(_) => todo!(),
    };

    tui::run(Router { workspaces })?;

    tui::restore_terminal()?;

    Ok(())
}
