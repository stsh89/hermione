mod commands;

use crate::{
    coordinator::workspaces::ListParameters,
    handlers::{self, workspaces::*},
    parameters,
    routes::workspaces::Route,
    Coordinator, Model, Result,
};

pub struct Router<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { coordinator } = self;

        match route {
            Route::Home => {
                let workspaces = coordinator.workspaces().list(ListParameters {
                    page_number: 0,
                    page_size: 1,
                    name_contains: "",
                })?;

                let Some(workspace) = workspaces.into_iter().next() else {
                    let handler = new::Handler {};
                    let model = handler.handle()?;

                    return Ok(Some(Box::new(model)));
                };

                let handler = handlers::workspaces::commands::list::Handler { coordinator };
                let model = handler.handle(parameters::workspaces::commands::list::Parameters {
                    workspace_id: workspace.id,
                    page_number: 0,
                    page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                    search_query: "".into(),
                    powershell_no_exit: false,
                })?;

                Ok(Some(Box::new(model)))
            }
            Route::Commands(route) => commands::Router { coordinator }.handle(route),
            Route::Create(parameters) => {
                let handler = create::Handler { coordinator };
                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { coordinator };
                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(paramters) => {
                let handler = edit::Handler { coordinator };
                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(list_parameters) => {
                let handler = list::Handler { coordinator };
                let model = handler.handle(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New => {
                let handler = new::Handler {};
                let model = handler.handle()?;

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { coordinator };
                let workspace = handler.handle(parameters)?;
                let model = list::Handler { coordinator }.handle(
                    parameters::workspaces::list::Parameters {
                        search_query: workspace.name,
                        page_number: 0,
                        page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
