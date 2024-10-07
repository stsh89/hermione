use crate::{
    handlers::workspaces::commands::*, integrations, parameters,
    routes::workspaces::commands::Route, Model, Result,
};

pub struct Router<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { workspaces } = self;

        match route {
            Route::Create(paramters) => {
                let handler = create::Handler { workspaces };
                let command = handler.handle(paramters)?;

                let model = list::Handler { workspaces }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: command.workspace_id,
                        search_query: Some(command.program),
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { workspaces };
                let workspace = handler.handle(parameters)?;

                let model = list::Handler { workspaces }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: workspace.id,
                        search_query: None,
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(parameters) => {
                let handler = edit::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Get(parameters) => {
                let handler = get::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(parameters) => {
                let handler = list::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New(parameters) => {
                let handler = new::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { workspaces };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
