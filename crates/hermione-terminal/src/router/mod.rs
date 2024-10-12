mod powershell;
mod workspaces;

use crate::{
    brokers,
    coordinator::workspaces::ListParameters,
    parameters::{self, workspaces::list::PAGE_SIZE},
    routes::{self, Route},
    Coordinator, Message, Model, Result,
};

pub struct Router {
    pub coordinator: Coordinator,
    pub powershell: brokers::powershell::Broker,
}

impl hermione_tui::Router for Router {
    type Route = Route;
    type Message = Message;

    fn handle_initial_route(&self) -> Result<Option<Box<Model>>> {
        let route = self.initial_route()?;

        self.handle(route)
    }

    fn handle(&self, route: Route) -> Result<Option<Box<Model>>> {
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

impl Router {
    fn initial_route(&self) -> Result<Route> {
        let mut workspaces = self.coordinator.workspaces().list(ListParameters {
            page_number: 0,
            page_size: PAGE_SIZE,
            name_contains: "",
        })?;

        workspaces.reverse();

        let Some(workspace) = workspaces.pop() else {
            return Ok(Route::Workspaces(routes::workspaces::Route::New));
        };

        Ok(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: workspace.id,
                    search_query: "".into(),
                    page_number: 0,
                    page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                },
            ),
        )))
    }
}
