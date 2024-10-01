use crate::parameters;

pub enum Route {
    ExecuteCommand(parameters::powershell::execute_command::Parameters),
    CopyToClipboard(parameters::powershell::copy_to_clipboard::Parameters),
    StartWindowsTerminal(parameters::powershell::start_windows_terminal::Parameters),
}
