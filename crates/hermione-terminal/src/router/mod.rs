mod powershell;
mod workspaces;

use crate::{
    integrations, parameters,
    routes::{self, Route},
    tui, Message, Model, Result,
};

pub struct Router {
    pub workspaces: integrations::core::workspaces::Client,
}

impl tui::Router for Router {
    type Route = Route;
    type Message = Message;

    fn handle_initial_route(&self) -> Result<Option<Box<Model>>> {
        let route = self.initial_route()?;

        self.handle(route)
    }

    fn handle(&self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { workspaces } = self;

        match route {
            Route::Workspaces(route) => workspaces::Router { workspaces }.handle(route),
            Route::Powershell(route) => powershell::Router { workspaces }.handle(route),
        }
    }
}

impl Router {
    fn initial_route(&self) -> Result<Route> {
        let mut workspaces = self.workspaces.list()?;
        workspaces.reverse();

        let Some(workspace) = workspaces.pop() else {
            return Ok(Route::Workspaces(routes::workspaces::Route::New));
        };

        Ok(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: workspace.id,
                    ..Default::default()
                },
            ),
        )))
    }
}
