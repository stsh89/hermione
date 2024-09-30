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
