use crate::{
    app::{Handle, Hook},
    clients::memories::Client,
    controllers,
    routes::{self, Route},
    Result,
};

pub struct Router {
    pub memories: Client,
}

impl Handle<Route> for Router {
    fn handle(&self, route: Route) -> Result<Option<Box<dyn Hook<Route>>>> {
        match route {
            Route::Workspaces(route) => self.handle_workspaces_route(route),
            Route::Powershell(route) => self.handle_powershell_route(route),
        }
    }
}

impl Router {
    fn handle_workspaces_commands_route(
        &self,
        route: routes::workspaces::commands::Route,
    ) -> Result<Option<Box<dyn Hook<Route>>>> {
        let memories = &self.memories;

        match route {
            routes::workspaces::commands::Route::Create(paramters) => {
                let handler = controllers::workspaces::commands::create::Handler { memories };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::Delete(parameters) => {
                let handler = controllers::workspaces::commands::delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::Edit(parameters) => {
                let handler = controllers::workspaces::commands::edit::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::Get(parameters) => {
                let handler = controllers::workspaces::commands::get::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::List(parameters) => {
                let handler = controllers::workspaces::commands::list::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::New(parameters) => {
                let handler = controllers::workspaces::commands::new::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::commands::Route::Update(parameters) => {
                let handler = controllers::workspaces::commands::update::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }

    fn handle_workspaces_route(
        &self,
        route: routes::workspaces::Route,
    ) -> Result<Option<Box<dyn Hook<Route>>>> {
        let Router { memories } = self;

        match route {
            routes::workspaces::Route::Commands(route) => {
                self.handle_workspaces_commands_route(route)
            }
            routes::workspaces::Route::Create(parameters) => {
                let handler = controllers::workspaces::create::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::Route::Delete(parameters) => {
                let handler = controllers::workspaces::delete::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::Route::Edit(paramters) => {
                let handler = controllers::workspaces::edit::Handler { memories };

                let model = handler.handle(paramters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::Route::List(list_parameters) => {
                let handler = controllers::workspaces::list::Handler { memories };

                let model = handler.handle(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::Route::New => {
                let handler = controllers::workspaces::new::Handler {};

                let model = handler.handle();

                Ok(Some(Box::new(model)))
            }
            routes::workspaces::Route::Update(parameters) => {
                let handler = controllers::workspaces::update::Handler { memories };

                let model = handler.handle(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }

    fn handle_powershell_route(
        &self,
        route: routes::powershell::Route,
    ) -> Result<Option<Box<dyn Hook<Route>>>> {
        match route {
            routes::powershell::Route::CopyToClipboard(parameters) => {
                let handler = controllers::powershell::copy_to_clipboard::Handler {
                    memories: &self.memories,
                };

                handler.handle(parameters)?;

                Ok(None)
            }
            routes::powershell::Route::ExecuteCommand(parameters) => {
                let handler = controllers::powershell::execute_command::Handler {
                    memories: &self.memories,
                };

                handler.handle(parameters)?;

                Ok(None)
            }
            routes::powershell::Route::StartWindowsTerminal(parameters) => {
                let handler = controllers::powershell::start_windows_terminal::Handler {};

                handler.handle(parameters)?;

                Ok(None)
            }
        }
    }
}
