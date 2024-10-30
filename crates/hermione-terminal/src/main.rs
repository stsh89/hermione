mod colors;
mod coordinator;
mod forms;
mod handlers;
mod layouts;
mod message;
mod models;
mod params;
mod presenters;
mod providers;
mod router;
mod routes;
mod smart_input;
mod themes;
mod widgets;

use coordinator::Coordinator;
use hermione_storage::StorageProvider;
use hermione_tracing::{NewTracerParameters, Tracer};
use providers::{clipboard::ClipboardProvider, system::SystemProvider};
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
    let app_path = hermione_storage::app_path()?;
    let powershell_client = hermione_powershell::PowerShellClient::new()?;
    let connection = StorageProvider::connect(&app_path)?;

    let coordinator = Coordinator {
        storage_provider: StorageProvider::new(&connection)?,
        clipboard_provider: ClipboardProvider {
            client: &powershell_client,
        },
        system_provider: SystemProvider {
            client: &powershell_client,
        },
    };

    let router = TerminalRouter { coordinator };

    let tracer = Tracer::new(NewTracerParameters {
        directory: &app_path,
    });

    let _guard = tracer.init_non_blocking()?;

    if let Err(err) = hermione_tui::run(router) {
        tracing::error!(error = ?err);
        return Err(err);
    };

    Ok(())
}
