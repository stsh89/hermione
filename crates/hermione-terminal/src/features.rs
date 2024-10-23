use crate::Result;
use hermione_powershell::{PowerShell, PowerShellParameters};

pub trait CopyToClipboard {
    fn copy_to_clipboard(&self, text: &str) -> Result<()>;
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self, working_directory: &str) -> Result<()>;
}

pub trait ExecuteCommand {
    fn execute_command(&self, parameters: ExecuteCommandParameters) -> Result<()>;
}

pub struct CopyToClipboardOperation<'a, T>
where
    T: CopyToClipboard,
{
    pub clipboard_provider: &'a T,
}

pub struct ExecuteCommandOperation<'a, T>
where
    T: ExecuteCommand,
{
    pub executor: &'a T,
}

pub struct OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub windows_terminal_provider: &'a T,
}

pub struct ExecuteCommandParameters<'a> {
    /// Executes the specified commands (and any parameters) as though they
    /// were typed at the PowerShell command prompt, and then exits, unless the
    /// NoExit parameter is specified.
    pub command: &'a str,

    /// Does not exit after running startup commands.
    pub no_exit: bool,

    /// Sets the initial working directory by executing at startup.
    pub working_directory: &'a str,
}

impl<'a, T> CopyToClipboardOperation<'a, T>
where
    T: CopyToClipboard,
{
    pub fn execute(&self, text: &str) -> Result<()> {
        self.clipboard_provider.copy_to_clipboard(text)?;

        Ok(())
    }
}

impl<'a, T> ExecuteCommandOperation<'a, T>
where
    T: ExecuteCommand,
{
    pub fn execute(&self, parameters: ExecuteCommandParameters) -> Result<()> {
        self.executor.execute_command(parameters)?;

        Ok(())
    }
}

impl<'a, T> OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub fn execute(&self, working_directory: &str) -> Result<()> {
        self.windows_terminal_provider
            .open_windows_terminal(working_directory)?;

        Ok(())
    }
}

impl CopyToClipboard for PowerShell {
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.copy_to_clipboard(text)?;

        Ok(())
    }
}

impl ExecuteCommand for PowerShell {
    fn execute_command(&self, parameters: ExecuteCommandParameters) -> Result<()> {
        let ExecuteCommandParameters {
            command,
            no_exit,
            working_directory,
        } = parameters;

        self.open_windows_terminal(Some(PowerShellParameters {
            command: Some(command),
            no_exit,
            working_directory: Some(working_directory),
        }))?;

        Ok(())
    }
}

impl OpenWindowsTerminal for PowerShell {
    fn open_windows_terminal(&self, working_directory: &str) -> Result<()> {
        self.open_windows_terminal(Some(PowerShellParameters {
            command: None,
            no_exit: false,
            working_directory: Some(working_directory),
        }))?;

        Ok(())
    }
}
