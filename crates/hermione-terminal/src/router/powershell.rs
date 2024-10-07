use crate::{handlers::powershell::*, integrations, routes::powershell::Route, Model, Result};

pub struct Router<'a> {
    pub workspaces: &'a integrations::core::workspaces::Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { workspaces } = self;

        match route {
            Route::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler { workspaces };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::ExecuteCommand(parameters) => {
                let handler = execute_command::Handler { workspaces };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::StartWindowsTerminal(parameters) => {
                let handler = start_windows_terminal::Handler {};

                handler.handle(parameters)?;

                Ok(None)
            }
        }
    }
}
