mod brokers;
mod colors;
mod components;
mod coordinator;
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

use coordinator::Coordinator;
use router::Router;
use routes::Route;

pub use message::Message;

type Error = anyhow::Error;
type Model = dyn tui::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_path = hermione_terminal_directory::path()?;

    tui::install_panic_hook();
    logs::init(&app_path)?;

    tui::run(Router {
        coordinator: Coordinator::new(&app_path)?,
        powershell: brokers::powershell::Broker::new()?,
    })?;

    tui::restore_terminal()?;

    Ok(())
}
