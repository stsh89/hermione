use crate::providers::{open_windows_terminal_command, PowerShellClient, PowerShellParameters};
use hermione_nexus::{
    services::{RunProgram, SystemProvider},
    Result,
};

pub struct System<'a> {
    pub client: &'a PowerShellClient,
    pub no_exit: bool,
    pub working_directory: Option<&'a str>,
}

impl SystemProvider for System<'_> {}

impl RunProgram for System<'_> {
    fn run_program(&self, program: &str) -> Result<()> {
        let command = open_windows_terminal_command(Some(PowerShellParameters {
            command: Some(program),
            no_exit: self.no_exit,
            working_directory: self.working_directory,
        }));

        self.client.execute(&command)?;

        Ok(())
    }
}
