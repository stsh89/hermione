use crate::{CommandPresenter, CommandsCoordinator, Result};
use hermione_powershell::{PowerShellParameters, PowerShellProvider};

pub trait CopyToClipboard {
    fn copy_to_clipboard(&self, text: &str) -> Result<()>;
}

pub trait RunProgram {
    fn run(&self, parameters: RunProgramParameters) -> Result<()>;
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self, working_directory: &str) -> Result<()>;
}

pub trait TrackCommandExecutionTime {
    fn track(&self, command: &CommandPresenter) -> Result<()>;
}

pub struct CopyToClipboardOperation<'a, T>
where
    T: CopyToClipboard,
{
    pub clipboard_provider: &'a T,
}

pub struct ExecuteCommandOperation<'a, R, T>
where
    R: RunProgram,
    T: TrackCommandExecutionTime,
{
    pub runner: &'a R,
    pub tracker: &'a T,
}

pub struct OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub windows_terminal_provider: &'a T,
}

pub struct ExecuteCommandParameters<'a> {
    pub command: &'a CommandPresenter,
    pub no_exit: bool,
    pub working_directory: &'a str,
}

pub struct RunProgramParameters<'a> {
    pub program: &'a str,
    pub no_exit: bool,
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

impl<'a, R, T> ExecuteCommandOperation<'a, R, T>
where
    R: RunProgram,
    T: TrackCommandExecutionTime,
{
    pub fn execute(&self, parameters: ExecuteCommandParameters) -> Result<()> {
        let ExecuteCommandParameters {
            command,
            no_exit,
            working_directory,
        } = parameters;

        self.runner.run(RunProgramParameters {
            program: &command.program,
            no_exit,
            working_directory,
        })?;

        self.tracker.track(command)
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

impl CopyToClipboard for PowerShellProvider {
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.copy_to_clipboard(text)?;

        Ok(())
    }
}

impl RunProgram for PowerShellProvider {
    fn run(&self, parameters: RunProgramParameters) -> Result<()> {
        let RunProgramParameters {
            program,
            no_exit,
            working_directory,
        } = parameters;

        self.open_windows_terminal(Some(PowerShellParameters {
            command: Some(program),
            no_exit,
            working_directory: Some(working_directory),
        }))?;

        Ok(())
    }
}

impl OpenWindowsTerminal for PowerShellProvider {
    fn open_windows_terminal(&self, working_directory: &str) -> Result<()> {
        self.open_windows_terminal(Some(PowerShellParameters {
            command: None,
            no_exit: false,
            working_directory: Some(working_directory),
        }))?;

        Ok(())
    }
}

impl TrackCommandExecutionTime for CommandsCoordinator {
    fn track(&self, command: &CommandPresenter) -> Result<()> {
        self.track_execution_time(command)?;

        Ok(())
    }
}
