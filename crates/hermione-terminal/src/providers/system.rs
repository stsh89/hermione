use hermione_ops::{
    extensions::{
        OpenWindowsTerminal, OpenWindowsTerminalParameters, RunProgram, RunProgramParameters,
    },
    Result,
};
use hermione_powershell::{PowerShellClient, PowerShellParameters};

pub struct SystemProvider<'a> {
    pub client: &'a PowerShellClient,
}

impl RunProgram for SystemProvider<'_> {
    fn run(&self, parameters: RunProgramParameters) -> Result<()> {
        let RunProgramParameters {
            program,
            no_exit,
            working_directory,
        } = parameters;

        self.client
            .open_windows_terminal(Some(PowerShellParameters {
                command: Some(program),
                no_exit,
                working_directory: Some(working_directory),
            }))
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl OpenWindowsTerminal for SystemProvider<'_> {
    fn open_windows_terminal(&self, parameters: OpenWindowsTerminalParameters) -> Result<()> {
        let OpenWindowsTerminalParameters { working_directory } = parameters;

        self.client
            .open_windows_terminal(Some(PowerShellParameters {
                command: None,
                no_exit: false,
                working_directory: Some(working_directory),
            }))
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}
