mod copy_to_clipboard;
mod execute_command;
mod start_windows_terminal;

use crate::{app::Hook, clients::memories::Client, router::powershell::Router, types::Result};

pub struct Controller<'a> {
    pub memories: &'a Client,
}

impl<'a> Controller<'a> {
    pub fn run(&self, route: Router) -> Result<Option<Box<dyn Hook>>> {
        let Controller { memories } = self;

        match route {
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
