mod colors;
mod coordinator;
mod forms;
mod handlers;
mod layouts;
mod message;
mod models;
mod params;
mod presenters;
mod router;
mod routes;
mod smart_input;
mod widgets;

use coordinator::Coordinator;
use hermione_storage::file_system::{FileSystemProvider, TERMINAL_APP_LOGS_FILE_NAME_PREFIX};
use hermione_tracing::{NewTracerParameters, Tracer};
use router::TerminalRouter;

pub(crate) use handlers::*;
pub(crate) use message::*;
pub(crate) use models::*;
pub(crate) use params::*;
pub(crate) use presenters::*;
pub(crate) use routes::*;

type Error = anyhow::Error;
type Result<T> = anyhow::Result<T>;

fn main() -> Result<()> {
    let file_system = FileSystemProvider::new().map_err(|err| anyhow::anyhow!(err))?;
    let coordinator = Coordinator::new(&file_system.database_file_path())?;
    let router = TerminalRouter { coordinator };

    let tracer = Tracer::new(NewTracerParameters {
        directory: file_system.location().into(),
        filename_prefix: TERMINAL_APP_LOGS_FILE_NAME_PREFIX,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = hermione_tui::run(router) {
        tracing::error!(error = ?err);
        return Err(err);
    };

    Ok(())
}
