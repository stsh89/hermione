mod name;
mod program;

pub use name::*;
pub use program::*;

use crate::{DateTime, Error, Id, Result};

pub struct Entity {
    last_execute_time: Option<DateTime>,
    id: Option<Id>,
    name: Name,
    program: Program,
    workspace_id: Id,
}

pub struct LoadParameters {
    pub last_execute_time: Option<DateTime>,
    pub id: Id,
    pub name: Name,
    pub program: Program,
    pub workspace_id: Id,
}

pub struct NewParameters {
    pub name: Name,
    pub program: Program,
    pub workspace_id: Id,
}

pub struct ScopedId {
    pub workspace_id: Id,
    pub id: Id,
}

impl Entity {
    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn change_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn last_execute_time(&self) -> Option<&DateTime> {
        self.last_execute_time.as_ref()
    }

    pub fn load(parameters: LoadParameters) -> Self {
        let LoadParameters {
            id,
            last_execute_time: execute_time,
            name,
            program,
            workspace_id,
        } = parameters;

        Self {
            last_execute_time: execute_time,
            id: Some(id),
            name,
            program,
            workspace_id,
        }
    }

    pub fn id(&self) -> Option<Id> {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(parameters: NewParameters) -> Self {
        let NewParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        Self {
            last_execute_time: None,
            id: None,
            name,
            program,
            workspace_id,
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn set_id(&mut self, id: Id) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::Internal("Command id is already set".to_string()));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn workspace_id(&self) -> Id {
        self.workspace_id
    }
}
