use crate::{
    clients::memories::Client, controllers::powershell::*, routes::powershell::Route, Model, Result,
};

pub struct Router<'a> {
    pub memories: &'a Client,
}

impl<'a> Router<'a> {
    pub fn handle(self, route: Route) -> Result<Option<Box<Model>>> {
        let Router { memories } = self;

        match route {
            Route::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler { memories };

                handler.handle(parameters)?;

                Ok(None)
            }
            Route::ExecuteCommand(parameters) => {
                let handler = execute_command::Handler { memories };

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
