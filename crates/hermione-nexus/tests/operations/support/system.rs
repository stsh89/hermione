use eyre::Report;
use hermione_nexus::{
    services::{
        InvokeCommand, InvokeCommandParameters, SetClipboardContent, SetLocation, SystemService,
    },
    Error,
};
use std::sync::{PoisonError, RwLock};

#[derive(Default)]
pub struct MockSystem {
    pub program: RwLock<Option<String>>,
    pub location: RwLock<Option<String>>,
    pub clipboard: RwLock<Option<String>>,
}

impl SystemService for MockSystem {}

impl InvokeCommand for MockSystem {
    fn invoke_command(&self, parameters: InvokeCommandParameters) -> Result<(), Error> {
        let InvokeCommandParameters { command, location } = parameters;

        set_location(self, location).map_err(system_error)?;
        set_program(self, command).map_err(system_error)?;

        Ok(())
    }
}

impl SetClipboardContent for MockSystem {
    fn set_clipboard_content(&self, text: &str) -> Result<(), Error> {
        set_clipboard(self, text).map_err(system_error)
    }
}

impl SetLocation for MockSystem {
    fn set_location(&self, location: Option<&str>) -> Result<(), Error> {
        set_location(self, location).map_err(system_error)
    }
}

fn set_clipboard(system: &MockSystem, text: &str) -> Result<(), Report> {
    let mut content = system.clipboard.write().map_err(report_from_poison)?;

    *content = Some(text.to_string());

    Ok(())
}

fn set_location(system: &MockSystem, location: Option<&str>) -> Result<(), Report> {
    let mut system_location = system.location.write().map_err(report_from_poison)?;

    *system_location = location.map(ToString::to_string);

    Ok(())
}

fn set_program(system: &MockSystem, program: &str) -> Result<(), Report> {
    let mut system_program = system.program.write().map_err(report_from_poison)?;

    *system_program = Some(program.to_string());

    Ok(())
}

fn report_from_poison<T>(err: PoisonError<T>) -> Report {
    Report::msg(err.to_string())
}

fn system_error(err: Report) -> Error {
    Error::system(err)
}
