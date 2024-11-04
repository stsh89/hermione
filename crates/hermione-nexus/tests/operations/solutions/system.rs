use hermione_nexus::{
    services::{RunProgram, SystemProvider},
    Error,
};
use std::sync::{PoisonError, RwLock};

#[derive(thiserror::Error, Debug)]
pub enum MockSystemError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct MockSystemProvider {
    program: RwLock<Option<String>>,
}

impl MockSystemProvider {
    pub fn last_executed_program(&self) -> Result<Option<String>, MockSystemError> {
        let program = self.program.read()?;

        Ok(program.clone())
    }

    pub fn new() -> Self {
        Self {
            program: RwLock::new(None),
        }
    }

    pub fn set_program(&self, program: &str) -> Result<(), MockSystemError> {
        *self.program.write()? = Some(program.to_string());

        Ok(())
    }
}

impl<T> From<PoisonError<T>> for MockSystemError {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockAccess(err.to_string())
    }
}

impl From<MockSystemError> for Error {
    fn from(err: MockSystemError) -> Self {
        Error::System(eyre::Error::new(err))
    }
}

impl SystemProvider for MockSystemProvider {}

impl RunProgram for MockSystemProvider {
    fn run_program(&self, program: &str) -> Result<(), Error> {
        self.set_program(program)?;

        Ok(())
    }
}
