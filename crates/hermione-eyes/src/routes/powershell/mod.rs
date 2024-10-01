pub mod copy_to_clipboard;
pub mod execute_command;
pub mod start_windows_terminal;

use crate::{app::Hook, clients::memories::Client, Result};

pub enum Router {
    ExecuteCommand(execute_command::Parameters),
    CopyToClipboard(copy_to_clipboard::Parameters),
    StartWindowsTerminal(start_windows_terminal::Parameters),
}

pub struct RouterParameters<'a> {
    pub memories: &'a Client,
}

impl Router {
    pub fn handle(self, parameters: RouterParameters) -> Result<Option<Box<dyn Hook>>> {
        let RouterParameters { memories } = parameters;

        match self {
            Router::CopyToClipboard(parameters) => {
                let handler = copy_to_clipboard::Handler { memories };

                handler.handle(parameters)?;

                Ok(None)
            }
            Router::ExecuteCommand(parameters) => {
                let handler = execute_command::Handler { memories };

                handler.handle(parameters)?;

                Ok(None)
            }
            Router::StartWindowsTerminal(parameters) => {
                let handler = start_windows_terminal::Handler {};

                handler.handle(parameters)?;

                Ok(None)
            }
        }
    }
}
