use eyre::eyre;
use hermione_nexus::{
    services::{ExecuteProgram, SetLocation, SystemService},
    Error, Result,
};
use std::sync::{PoisonError, RwLock};

#[derive(Default)]
pub struct MockSystem {
    pub program: RwLock<Option<String>>,
    pub location: RwLock<Option<String>>,
}

impl SystemService for MockSystem {}

impl ExecuteProgram for MockSystem {
    fn execute_program(&self, program: &str) -> Result<()> {
        set_program(self, program)
            .map_err(|err| Error::System(err.wrap_err("Failed to execute program")))
    }
}

impl SetLocation for MockSystem {
    fn set_location(&self, location: Option<&str>) -> Result<()> {
        set_location(self, location)
            .map_err(|err| Error::System(err.wrap_err("Failed to set location")))
    }
}

fn memory_write_access_error<T>(_err: PoisonError<T>) -> eyre::Error {
    eyre!("Write memory access failure")
}

fn set_location(system: &MockSystem, location: Option<&str>) -> eyre::Result<()> {
    let mut system_location = system.location.write().map_err(memory_write_access_error)?;

    *system_location = location.map(ToString::to_string);

    Ok(())
}

fn set_program(system: &MockSystem, program: &str) -> eyre::Result<()> {
    let mut system_program = system.program.write().map_err(memory_write_access_error)?;

    *system_program = Some(program.to_string());

    Ok(())
}
