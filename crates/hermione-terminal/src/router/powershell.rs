use crate::{
    clients, handlers::powershell::*, routes::powershell::Route, BoxedModel, Coordinator, Result,
};

pub struct Router<'a> {
    pub coordinator: &'a Coordinator,
    pub powershell: &'a clients::powershell::PowerShell,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<BoxedModel>> {
        let Router {
            coordinator,
            powershell,
        } = self;

        match route {
            Route::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler {
                    coordinator,
                    powershell,
                };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::ExecuteCommand(parameters) => {
                let handler = execute_command::Handler {
                    coordinator,
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
