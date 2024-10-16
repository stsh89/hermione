mod clients;
mod colors;
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
mod smart_input;
mod widgets;

use clients::powershell::PowerShell;
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
    let coordinator = Coordinator::new(&app_path)?;
    let powershell = PowerShell::new()?;
    let route = initial_route(&coordinator)?;

    let router = Router {
        coordinator,
        powershell,
    };

    let Some(model) = router.dispatch(route)? else {
        return Ok(());
    };

    logs::init(&app_path)?;

    hermione_tui::install_panic_hook();
    hermione_tui::run(router, model)?;
    hermione_tui::restore_terminal()?;

    Ok(())
}

pub fn initial_route(coordinator: &Coordinator) -> Result<Route> {
    use coordinator::workspaces::ListParameters;

    let workspaces = coordinator.workspaces().list(ListParameters {
        page_number: 0,
        page_size: 1,
        name_contains: "",
    })?;

    let Some(workspace) = workspaces.into_iter().next() else {
        return Ok(Route::Workspaces(routes::workspaces::Route::New));
    };

    Ok(parameters::workspaces::commands::list::Parameters {
        workspace_id: workspace.id,
        search_query: "".into(),
        page_number: 0,
        page_size: parameters::workspaces::commands::list::PAGE_SIZE,
        powershell_no_exit: false,
    }
    .into())
}
