use std::{rc::Rc, str::FromStr};

use crate::{core, Connection, Result};
use chrono::{DateTime, Utc};
use hermione_core::{
    entities::command::{Entity, LoadParameters, Name, NewParameters, Program, ScopedId},
    operations::workspaces::commands::{
        create, delete, find, get, list, track_execution_time, update,
    },
    Id,
};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, workspace_id: &str, id: &str) -> Result<()>;
    fn find(&self, workspace_id: &str, id: &str) -> Result<Option<Dto>>;
    fn get(&self, workspace_id: &str, id: &str) -> Result<Dto>;
    fn list(&self, parameters: ListParameters) -> Result<Vec<Dto>>;
    fn track_execution_time(&self, workspace_id: &str, command_id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct Client {
    inner: core::workspaces::commands::Client,
}

pub struct Dto {
    pub id: String,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct ListParameters<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

impl Operations for Client {
    fn create(&self, data: Dto) -> Result<Dto> {
        let command = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(Dto::from_entity(command))
    }

    fn delete(&self, workspace_id: &str, id: &str) -> Result<()> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        delete::Operation {
            deleter: &self.inner,
        }
        .execute(id)?;

        Ok(())
    }

    fn find(&self, workspace_id: &str, id: &str) -> Result<Option<Dto>> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let command = find::Operation {
            finder: &self.inner,
        }
        .execute(id)?;

        Ok(command.map(Dto::from_entity))
    }

    fn get(&self, workspace_id: &str, id: &str) -> Result<Dto> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let command = get::Operation {
            getter: &self.inner,
        }
        .execute(id)?;

        Ok(Dto::from_entity(command))
    }

    fn list(&self, parameters: ListParameters<'_>) -> Result<Vec<Dto>> {
        let ListParameters {
            program_contains,
            workspace_id,
            page_number,
            page_size,
        } = parameters;

        let commands = list::Operation {
            lister: &self.inner,
        }
        .execute(list::Parameters {
            page_number,
            page_size,
            program_contains,
            workspace_id: Id::from_str(workspace_id)?,
        })?;

        Ok(commands.into_iter().map(Dto::from_entity).collect())
    }

    fn track_execution_time(&self, workspace_id: &str, id: &str) -> Result<Dto> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        use hermione_core::operations::workspaces::commands::get::Get;
        let entity = self.inner.get(id)?;

        let entity = track_execution_time::Operation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(Dto::from_entity(entity))
    }

    fn update(&self, data: Dto) -> Result<Dto> {
        let command = update::Operation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(Dto::from_entity(command))
    }
}

impl Client {
    pub fn new(connection: Rc<Connection>) -> Self {
        let inner = core::workspaces::commands::Client::new(connection);

        Self { inner }
    }
}

impl Dto {
    fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id().map(|id| id.to_string()).unwrap_or_default(),
            name: entity.name().to_string(),
            last_execute_time: entity.last_execute_time().map(Into::into),
            program: entity.program().to_string(),
            workspace_id: entity.workspace_id().to_string(),
        }
    }

    fn load_entity(self) -> eyre::Result<Entity> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            name: Name::new(self.name),
            last_execute_time: self.last_execute_time.map(From::from),
            program: Program::new(self.program),
            workspace_id: Id::from_str(&self.workspace_id)?,
        }))
    }

    fn new_entity(self) -> eyre::Result<Entity> {
        Ok(Entity::new(NewParameters {
            name: Name::new(self.name),
            program: Program::new(self.program),
            workspace_id: Id::from_str(&self.workspace_id)?,
        }))
    }
}
