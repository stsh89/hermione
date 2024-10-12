mod breadcrumbs;
mod brokers;
mod colors;
mod components;
mod coordinator;
mod forms;
mod handlers;
mod layouts;
mod logs;
mod message;
mod models;
mod parameters;
mod presenters;
mod router;
mod routes;
mod widgets;

use coordinator::Coordinator;
use hermione_tui::app;
use message::Message;
use router::Router;
use routes::Route;

type Error = anyhow::Error;
type Model = dyn app::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let app_path = hermione_terminal_directory::path()?;

    hermione_tui::install_panic_hook();
    logs::init(&app_path)?;

    hermione_tui::run(Router {
        coordinator: Coordinator::new(&app_path)?,
        powershell: brokers::powershell::Broker::new()?,
    })?;

    hermione_tui::restore_terminal()?;

    Ok(())
}
