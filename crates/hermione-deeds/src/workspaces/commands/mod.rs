mod json;

use crate::Result;
use chrono::{DateTime, Utc};
use hermione_core::{
    entities::command::{Entity, LoadParameters, Name, NewParameters, Program, ScopedId},
    operations::workspaces::commands::{create, delete, get, list, track_execution_time, update},
    Id,
};
use std::{path::PathBuf, str::FromStr};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, workspace_id: &str, id: &str) -> Result<()>;
    fn get(&self, workspace_id: &str, id: &str) -> Result<Dto>;
    fn list(&self, workspace_id: &str) -> Result<Vec<Dto>>;
    fn track_execution_time(&self, workspace_id: &str, command_id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct Client {
    inner: json::Client,
}

pub struct Dto {
    pub id: String,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

impl Operations for Client {
    fn create(&self, data: Dto) -> Result<Dto> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(Dto::from_entity(workspace))
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

    fn get(&self, workspace_id: &str, id: &str) -> Result<Dto> {
        let id = ScopedId {
            workspace_id: Id::from_str(workspace_id)?,
            id: Id::from_str(id)?,
        };

        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(id)?;

        Ok(Dto::from_entity(workspace))
    }

    fn list(&self, workspace_id: &str) -> Result<Vec<Dto>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
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
        let workspace = update::Operation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(Dto::from_entity(workspace))
    }
}

impl Client {
    pub fn new(path: PathBuf) -> Result<Self> {
        let inner = json::Client::new(path)?;

        Ok(Self { inner })
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
