use super::WorkspaceId;
use crate::{Error, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Command {
    id: CommandId,
    last_execute_time: Option<DateTime<Utc>>,
    program: CommandProgram,
    name: CommandName,
    workspace_id: WorkspaceId,
}

pub struct CommandParameters {
    pub id: Uuid,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub program: String,
    pub name: String,
    pub workspace_id: WorkspaceId,
}

#[derive(Clone, Debug)]
pub struct CommandId(Uuid);

#[derive(Clone)]
struct CommandProgram {
    value: String,
}

#[derive(Clone)]
struct CommandName {
    value: String,
}

impl Command {
    pub fn id(&self) -> &CommandId {
        &self.id
    }

    pub fn last_execute_time(&self) -> Option<&DateTime<Utc>> {
        self.last_execute_time.as_ref()
    }

    pub fn program(&self) -> &str {
        &self.program.value
    }

    pub fn name(&self) -> &str {
        &self.name.value
    }

    pub fn new(parameters: CommandParameters) -> Result<Self> {
        let CommandParameters {
            id,
            last_execute_time,
            program,
            name,
            workspace_id,
        } = parameters;

        Ok(Self {
            id: CommandId::new(id)?,
            last_execute_time,
            program: CommandProgram { value: program },
            name: CommandName { value: name },
            workspace_id,
        })
    }

    pub fn set_program(&mut self, program: String) {
        self.program = CommandProgram { value: program };
    }

    pub fn set_name(&mut self, name: String) {
        self.name = CommandName { value: name };
    }

    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
    }
}

impl CommandId {
    fn new(value: Uuid) -> Result<Self> {
        if value.is_nil() {
            return Err(Error::Internal("Invalid command ID".to_string()));
        }

        Ok(Self(value))
    }
}

impl std::ops::Deref for CommandId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Uuid> for CommandId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
