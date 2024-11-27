use eyre::Report;
use hermione_nexus::{
    services::{
        InvokeCommand, InvokeCommandParameters, SetClipboardContent, SetLocation, SystemService,
    },
    Error,
};
use std::sync::RwLock;

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

        set_location(self, location).map_err(Error::system)?;
        set_program(self, command).map_err(Error::system)?;

        Ok(())
    }
}

impl SetClipboardContent for MockSystem {
    fn set_clipboard_content(&self, text: &str) -> Result<(), Error> {
        set_clipboard_content(self, text).map_err(Error::system)
    }
}

impl SetLocation for MockSystem {
    fn set_location(&self, location: Option<&str>) -> Result<(), Error> {
        set_location(self, location).map_err(Error::system)
    }
}

fn set_clipboard_content(system: &MockSystem, text: &str) -> Result<(), Report> {
    let mut content = system
        .clipboard
        .write()
        .map_err(|err| Report::msg(err.to_string()))?;

    *content = Some(text.to_string());

    Ok(())
}

fn set_location(system: &MockSystem, location: Option<&str>) -> Result<(), Report> {
    let mut system_location = system
        .location
        .write()
        .map_err(|err| Report::msg(err.to_string()))?;

    *system_location = location.map(ToString::to_string);

    Ok(())
}

fn set_program(system: &MockSystem, program: &str) -> Result<(), Report> {
    let mut system_program = system
        .program
        .write()
        .map_err(|err| Report::msg(err.to_string()))?;

    *system_program = Some(program.to_string());

    Ok(())
}
