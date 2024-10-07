mod commands;

use crate::{
    handlers::{self, workspaces::*},
    integrations, parameters,
    routes::workspaces::Route,
    Model, Result,
};

pub struct Router<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { workspaces } = self;

        match route {
            Route::Commands(route) => commands::Router { workspaces }.handle(route),
            Route::Create(parameters) => {
                let handler = create::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(paramters) => {
                let handler = edit::Handler { workspaces };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(list_parameters) => {
                let handler = list::Handler { workspaces };

                let model = handler.handle(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New => {
                let handler = new::Handler {};

                let model = handler.handle();

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { workspaces };

                let workspace = handler.handle(parameters)?;

                let model = handlers::workspaces::commands::list::Handler { workspaces }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: workspace.id,
                        search_query: None,
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
