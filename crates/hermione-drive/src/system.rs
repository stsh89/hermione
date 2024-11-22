use hermione_internals::powershell::{self, PowerShellParameters, PowerShellProcess};
use hermione_nexus::{
    services::{InvokeCommand, InvokeCommandParameters, SetLocation, SystemService},
    Result,
};

pub struct System<'a> {
    process: &'a PowerShellProcess,
    no_exit: bool,
}

impl<'a> System<'a> {
    pub fn new(process: &'a PowerShellProcess) -> Self {
        System {
            process,
            no_exit: true,
        }
    }

    pub fn set_no_exit(&mut self, no_exit: bool) {
        self.no_exit = no_exit;
    }
}

impl SystemService for System<'_> {}

impl InvokeCommand for System<'_> {
    fn invoke_command(&self, parameters: InvokeCommandParameters) -> Result<()> {
        let InvokeCommandParameters {
            command,
            location: working_directory,
        } = parameters;

        powershell::open_windows_terminal(
            self.process,
            Some(PowerShellParameters {
                command: Some(command),
                no_exit: self.no_exit,
                working_directory,
            }),
        )
    }
}

impl SetLocation for System<'_> {
    fn set_location(&self, working_directory: Option<&str>) -> Result<()> {
        powershell::open_windows_terminal(
            self.process,
            Some(PowerShellParameters {
                command: None,
                no_exit: self.no_exit,
                working_directory,
            }),
        )
    }
}
