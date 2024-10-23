use crate::{PowerShellParameters, PowerShellProvider};
use hermione_core::{
    commands::{CopyToClipboard, RunProgram, RunProgramParameters},
    extensions::{OpenWindowsTerminal, OpenWindowsTerminalParameters},
    Result,
};

impl CopyToClipboard for PowerShellProvider {
    fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        self.copy_to_clipboard(text).map_err(eyre::Error::new)?;

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
        }))
        .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl OpenWindowsTerminal for PowerShellProvider {
    fn open_windows_terminal(&self, parameters: OpenWindowsTerminalParameters) -> Result<()> {
        let OpenWindowsTerminalParameters { working_directory } = parameters;

        self.open_windows_terminal(Some(PowerShellParameters {
            command: None,
            no_exit: false,
            working_directory: Some(working_directory),
        }))
        .map_err(eyre::Error::new)?;

        Ok(())
    }
}
