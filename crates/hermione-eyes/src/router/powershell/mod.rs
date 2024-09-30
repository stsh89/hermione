pub enum Router {
    ExecuteCommand(ExecuteCommandParameters),
    CopyToClipboard(CopyToClipboardParameters),
    StartWindowsTerminal(StartWindowsTerminalParameters),
}

pub struct ExecuteCommandParameters {
    pub command_id: String,
    pub workspace_id: String,
    pub powershell_no_exit: bool,
}

pub struct CopyToClipboardParameters {
    pub command_id: String,
    pub workspace_id: String,
}

pub struct StartWindowsTerminalParameters {
    pub working_directory: Option<String>,
}

macro_rules! from_parameters {
    ($action:ident, $parameters:ident) => {
        impl From<$parameters> for Router {
            fn from(parameters: $parameters) -> Self {
                Router::$action(parameters)
            }
        }
    };
}

from_parameters!(ExecuteCommand, ExecuteCommandParameters);
from_parameters!(CopyToClipboard, CopyToClipboardParameters);
from_parameters!(StartWindowsTerminal, StartWindowsTerminalParameters);
