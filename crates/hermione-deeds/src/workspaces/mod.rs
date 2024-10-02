mod json;

pub mod commands;

use crate::Result;
use chrono::{DateTime, Utc};
use hermione_memories::{
    entities::workspace::{Entity, LoadParameters, Location, Name, NewParameters},
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    Id,
};
use std::{path::PathBuf, str::FromStr};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, id: &str) -> Result<()>;
    fn get(&self, id: &str) -> Result<Dto>;
    fn list(&self) -> Result<Vec<Dto>>;
    fn track_access_time(&self, id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct Client {
    inner: json::Client,
}

pub struct Dto {
    pub id: String,
    pub last_access_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub name: String,
}

impl Operations for Client {
    fn create(&self, data: Dto) -> Result<Dto> {
        let workspace = create::Operation {
            creator: &self.inner,
        }
        .execute(data.new_entity())?;

        Ok(Dto::from_entity(workspace))
    }

    fn delete(&self, id: &str) -> Result<()> {
        delete::Operation {
            deleter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(())
    }

    fn get(&self, id: &str) -> Result<Dto> {
        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(Dto::from_entity(workspace))
    }

    fn list(&self) -> Result<Vec<Dto>> {
        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute()?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
    }

    fn track_access_time(&self, id: &str) -> Result<Dto> {
        use hermione_memories::operations::workspaces::get::Get;
        let entity = self.inner.get(Id::from_str(id)?)?;

        let entity = track_access_time::Operation {
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
            last_access_time: entity.last_access_time().map(Into::into),
            location: entity.location().map(ToString::to_string),
            name: entity.name().to_string(),
        }
    }

    fn load_entity(self) -> Result<Entity> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            name: Name::new(self.name),
            location: self.location.map(Location::new),
            last_access_time: self.last_access_time.map(From::from),
        }))
    }

    fn new_entity(self) -> Entity {
        Entity::new(NewParameters {
            name: Name::new(self.name),
            location: self.location.map(Location::new),
        })
    }
}
