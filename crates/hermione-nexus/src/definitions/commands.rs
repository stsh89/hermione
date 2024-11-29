use crate::{definitions::WorkspaceId, Error, Result};
use chrono::{DateTime, Utc};
use eyre::eyre;
use std::fmt::{self, Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Clone)]
pub struct Command {
    id: CommandId,
    last_execute_time: Option<DateTime<Utc>>,
    name: CommandName,
    program: CommandProgram,
    workspace_id: WorkspaceId,
}

pub struct CommandParameters {
    pub id: Uuid,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
    pub fn id(&self) -> CommandId {
        self.id
    }

    pub fn last_execute_time(&self) -> Option<&DateTime<Utc>> {
        self.last_execute_time.as_ref()
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

    pub fn program(&self) -> &str {
        &self.program.value
    }

    pub fn set_execute_time(&mut self, time: DateTime<Utc>) {
        self.last_execute_time = Some(time);
    }

    pub fn set_program(&mut self, program: String) {
        self.program = CommandProgram { value: program };
    }

    pub fn set_name(&mut self, name: String) {
        self.name = CommandName { value: name };
    }

    pub fn workspace_id(&self) -> WorkspaceId {
        self.workspace_id
    }
}

impl CommandId {
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn into_bytes(self) -> [u8; 16] {
        self.0.into_bytes()
    }

    pub fn new(id: Uuid) -> Result<Self> {
        if id.is_nil() {
            return Err(Error::invalid_argument(eyre!("Command ID cannot be nil")));
        }

        Ok(Self(id))
    }
}

impl Debug for CommandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "command:{}", self.0)
    }
}

impl Display for CommandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
