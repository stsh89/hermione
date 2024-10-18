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

use clients::powershell::PowerShell;
use coordinator::Coordinator;
use hermione_tui::app;
use message::Message;
use router::Router;
use routes::Route;
use std::path::PathBuf;

const LOGS_FILE_NAME: &str = "hermione.logs";
const INITIAL_ROUTE: Route = Route::Workspaces(routes::workspaces::Route::Home);

type Error = anyhow::Error;
type Model = dyn app::Model<Route = Route, Message = Message>;
type Result<T> = anyhow::Result<T>;

pub struct App {
    /// The path to the directory where all the files related to the Hermione app are stored.
    path: PathBuf,

    router: Router,
}

impl App {
    fn enable_tracing(self) -> Result<Self> {
        hermione_logs::init(&self.path.join(LOGS_FILE_NAME))?;

        Ok(self)
    }

    fn new() -> Result<Self> {
        let path = hermione_terminal_directory::path()?;
        let coordinator = Coordinator::new(&path)?;
        let powershell = PowerShell::new()?;

        let router = Router {
            coordinator,
            powershell,
        };

        Ok(Self { path, router })
    }

    fn run(self) -> Result<()> {
        let App { path: _, router } = self;

        let Some(model) = router.dispatch(INITIAL_ROUTE)? else {
            return Err(anyhow::anyhow!("Transparent initial route"));
        };

        hermione_tui::run(router, model)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    App::new()?.enable_tracing()?.run()
}
