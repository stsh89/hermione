use crate::{
    clients, controllers::workspaces::commands::*, parameters, routes::workspaces::commands::Route,
    Model, Result,
};

pub struct Router<'a> {
    pub memories: &'a clients::memories::Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { memories } = self;

        match route {
            Route::Create(paramters) => {
                let handler = create::Handler { memories };
                let command = handler.handle(paramters)?;

                let model = list::Handler { memories }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: command.workspace_id,
                        search_query: Some(command.program),
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(parameters) => {
                let handler = edit::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Get(parameters) => {
                let handler = get::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(parameters) => {
                let handler = list::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New(parameters) => {
                let handler = new::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
