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
use router::Router;

const LOGS_FILE_NAME_PREFIX: &str = "hermione-terminal-logs";

type BoxedModel = Box<dyn hermione_tui::Model<Message = Message, Route = routes::Route>>;
type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let directory = hermione_terminal_directory::path()?;

    let coordinator = Coordinator::new(&directory)?;
    let powershell = PowerShell::new()?;

    let router = Router {
        coordinator,
        powershell,
    };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &directory,
        filename_prefix: LOGS_FILE_NAME_PREFIX,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = hermione_tui::run(router) {
        tracing::error!(error = ?err);
        return Err(err);
    };

    Ok(())
}
