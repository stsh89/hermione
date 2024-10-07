mod commands;

use crate::{
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

                let model = handler.handle();

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { coordinator };

                let workspace = handler.handle(parameters)?;

                let model = handlers::workspaces::commands::list::Handler { coordinator }.handle(
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
