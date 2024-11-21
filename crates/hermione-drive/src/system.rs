use hermione_internals::powershell::{self, PowerShellParameters, PowerShellProcess};
use hermione_nexus::{
    services::{ExecuteProgram, SystemService},
    Result,
};

pub struct System<'a> {
    process: &'a PowerShellProcess,
    no_exit: bool,
    working_directory: Option<&'a str>,
}

impl<'a> System<'a> {
    pub fn new(process: &'a PowerShellProcess) -> Self {
        System {
            process,
            no_exit: true,
            working_directory: None,
        }
    }

    pub fn set_no_exit(&mut self, no_exit: bool) {
        self.no_exit = no_exit;
    }

    pub fn set_workking_directory(&mut self, working_directory: &'a str) {
        self.working_directory = Some(working_directory);
    }

    pub fn unset_working_directory(&mut self) {
        self.working_directory = None;
    }
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
        )
    }
}
