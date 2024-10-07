use crate::{
    brokers, clients::memories::Client, handlers::powershell::*, routes::powershell::Route, Model,
    Result,
};

pub struct Router<'a> {
    pub memories: &'a Client,
    pub powershell: &'a brokers::powershell::Broker,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router {
            memories,
            powershell,
        } = self;

        match route {
            Route::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler {
                    memories,
                    powershell,
                };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::ExecuteCommand(parameters) => {
                let handler = execute_command::Handler {
                    memories,
                    powershell,
                };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::StartWindowsTerminal(parameters) => {
                let handler = start_windows_terminal::Handler { powershell };

                handler.handle(parameters)?;

                Ok(None)
            }
        }
    }
}
