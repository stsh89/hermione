mod commands;

use crate::{
    clients,
    handlers::{self, workspaces::*},
    parameters,
    routes::workspaces::Route,
    Model, Result,
};

pub struct Router<'a> {
    pub memories: &'a clients::memories::Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { memories } = self;

        match route {
            Route::Commands(route) => commands::Router { memories }.handle(route),
            Route::Create(parameters) => {
                let handler = create::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(paramters) => {
                let handler = edit::Handler { memories };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(list_parameters) => {
                let handler = list::Handler { memories };

                let model = handler.handle(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New => {
                let handler = new::Handler {};

                let model = handler.handle();

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { memories };

                let workspace = handler.handle(parameters)?;

                let model = handlers::workspaces::commands::list::Handler { memories }.handle(
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
