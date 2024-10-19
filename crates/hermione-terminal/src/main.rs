mod clients;
mod colors;
mod coordinator;
mod forms;
mod handlers;
mod layouts;
mod message;
mod models;
mod parameters;
mod presenters;
mod router;
mod routes;
mod smart_input;
mod widgets;

use hermione_tracing::{NewTracerParameters, Tracer};
pub(crate) use message::*;

use clients::powershell::PowerShell;
use coordinator::Coordinator;
use hermione_tui::app;
use router::Router;
use routes::Route;

const LOGS_FILE_NAME_PREFIX: &str = "hermione-terminal-logs";
const INITIAL_ROUTE: Route = Route::Workspaces(routes::workspaces::Route::Home);

type Error = anyhow::Error;
type Model = dyn app::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let directory = hermione_terminal_directory::path()?;

    let coordinator = Coordinator::new(&directory)?;
    let powershell = PowerShell::new()?;

    let router = Router {
        coordinator,
        powershell,
    };

    let Some(model) = router.dispatch(INITIAL_ROUTE)? else {
        return Err(anyhow::anyhow!("Transparent initial route"));
    };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &directory,
        filename_prefix: LOGS_FILE_NAME_PREFIX,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = hermione_tui::run(router, model) {
        tracing::error!(error = ?err);
        return Err(err);
    };

    Ok(())
}
