mod name;
mod program;

pub use name::*;
pub use program::*;

use crate::types::shared::{DateTime, Error, Id, Result};

pub struct Entity {
    execute_time: Option<DateTime>,
    id: Option<Id>,
    name: Name,
    program: Program,
}

pub struct LoadParameters {
    pub last_execute_time: Option<DateTime>,
    pub id: Id,
    pub name: Name,
    pub program: Program,
}

pub struct NewParameters {
    pub name: Name,
    pub program: Program,
}

impl Entity {
    pub fn change_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn last_execute_time(&self) -> Option<&DateTime> {
        self.execute_time.as_ref()
    }

    pub fn id(&self) -> Result<Id> {
        self.id.ok_or(Error::DataLoss("Id not set".to_string()))
    }

    pub fn load(parameters: LoadParameters) -> Self {
        let LoadParameters {
            name,
            program,
            id,
            last_execute_time: execute_time,
        } = parameters;

        Self {
            execute_time,
            id: Some(id),
            name,
            program,
        }
    }

    pub fn new(parameters: NewParameters) -> Self {
        let NewParameters { name, program } = parameters;

        Self {
            execute_time: None,
            id: None,
            name,
            program,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn rename(&mut self, name: Name) {
        self.name = name;
    }

    pub fn set_id(&mut self, id: Id) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::AlreadyExists(format!(
                "Command with id {} already exists",
                id
            )));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn update_last_execute_time(&mut self) {
        self.execute_time = Some(DateTime::now());
    }
}
