mod powershell;
mod workspaces;

use crate::{
    clients,
    coordinator::{self, Coordinator},
    handlers, parameters,
    routes::Route,
    Message, Result,
};

type BoxedModel = Box<dyn hermione_tui::Model<Message = Message, Route = Route>>;

pub struct Router {
    pub coordinator: Coordinator,
    pub powershell: clients::powershell::PowerShell,
}

impl hermione_tui::Router for Router {
    type Route = Route;
    type Message = Message;

    fn default_model(&self) -> Result<BoxedModel> {
        let workspaces =
            self.coordinator
                .workspaces()
                .list(coordinator::workspaces::ListParameters {
                    page_number: 0,
                    page_size: 1,
                    name_contains: "",
                })?;

        let Some(workspace) = workspaces.into_iter().next() else {
            let handler = handlers::workspaces::new::Handler {};
            let model = handler.handle()?;

            return Ok(Box::new(model));
        };

        let handler = handlers::workspaces::commands::list::Handler {
            coordinator: &self.coordinator,
        };

        let model = handler.handle(parameters::workspaces::commands::list::Parameters {
            workspace_id: workspace.id,
            page_number: 0,
            page_size: parameters::workspaces::commands::list::PAGE_SIZE,
            search_query: "".into(),
            powershell_no_exit: false,
        })?;

        Ok(Box::new(model))
    }

    fn handle(&self, route: Route) -> Result<Option<BoxedModel>> {
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
