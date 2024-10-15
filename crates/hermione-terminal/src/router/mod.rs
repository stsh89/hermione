mod powershell;
mod workspaces;

use crate::{clients, coordinator::Coordinator, routes::Route, Message, Model, Result};
use hermione_tui::app;

pub struct Router {
    pub coordinator: Coordinator,
    pub powershell: clients::powershell::PowerShell,
}

impl app::Router for Router {
    type Route = Route;
    type Message = Message;

    fn handle(&self, route: Route) -> Result<Option<Box<Model>>> {
        self.dispatch(route)
    }
}

impl Router {
    pub fn dispatch(&self, route: Route) -> Result<Option<Box<Model>>> {
        let Router {
            coordinator,
            powershell,
        } = self;

        match route {
            Route::Workspaces(route) => workspaces::Router { coordinator }.handle(route),
            Route::Powershell(route) => powershell::Router {
                coordinator,
                powershell,
            }
            .handle(route),
        }
    }
}
