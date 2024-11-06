use crate::providers::powershell::{self, PowerShellParameters, PowerShellProcess};
use hermione_nexus::{
    services::{ExecuteProgram, SystemService},
    Result,
};

pub struct System<'a> {
    pub process: &'a PowerShellProcess,
    pub no_exit: bool,
    pub working_directory: Option<&'a str>,
}

impl SystemService for System<'_> {}

impl ExecuteProgram for System<'_> {
    fn execute_program(&self, program: &str) -> Result<()> {
        powershell::open_windows_terminal(
            self.process,
            Some(PowerShellParameters {
                command: Some(program),
                no_exit: self.no_exit,
                working_directory: self.working_directory,
            }),
        )?;

        Ok(())
    }
}
