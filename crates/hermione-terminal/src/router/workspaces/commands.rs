use crate::{
    coordinator::Coordinator,
    handlers::workspaces::commands::*,
    parameters::{self, workspaces::commands::list::PAGE_SIZE},
    routes::workspaces::commands::Route,
    Model, Result,
};

pub struct Router<'a> {
    pub coordinator: &'a Coordinator,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { coordinator } = self;

        match route {
            Route::Create(paramters) => {
                let handler = create::Handler { coordinator };
                let command = handler.handle(paramters)?;

                let model = list::Handler { coordinator }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: command.workspace_id,
                        search_query: command.program,
                        page_number: 0,
                        page_size: PAGE_SIZE,
                        powershell_no_exit: false,
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
            Route::Delete(parameters) => {
                let handler = delete::Handler { coordinator };
                let workspace = handler.handle(parameters)?;

                let model = list::Handler { coordinator }.handle(
                    parameters::workspaces::commands::list::Parameters {
                        workspace_id: workspace.id,
                        search_query: "".to_string(),
                        page_number: 0,
                        page_size: PAGE_SIZE,
                        powershell_no_exit: false,
                    },
                )?;

                Ok(Some(Box::new(model)))
            }
            Route::Edit(parameters) => {
                let handler = edit::Handler { coordinator };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::List(parameters) => {
                let handler = list::Handler { coordinator };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::New(parameters) => {
                let handler = new::Handler { coordinator };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            Route::Update(parameters) => {
                let handler = update::Handler { coordinator };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
