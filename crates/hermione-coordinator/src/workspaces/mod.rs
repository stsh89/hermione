pub mod commands;

use crate::{core, Connection, Result};
use chrono::{DateTime, Utc};
use hermione_core::{
    entities::workspace::{Entity, LoadParameters, Location, Name, NewParameters},
    operations::workspaces::{create, delete, find, get, import, list, track_access_time, update},
    Id,
};
use std::{rc::Rc, str::FromStr};

pub trait Operations {
    fn create(&self, data: Dto) -> Result<Dto>;
    fn delete(&self, id: &str) -> Result<()>;
    fn find(&self, id: &str) -> Result<Option<Dto>>;
    fn get(&self, id: &str) -> Result<Dto>;
    fn import(&self, data: Dto) -> Result<Dto>;
    fn list(&self, parameters: ListParameters) -> Result<Vec<Dto>>;
    fn track_access_time(&self, id: &str) -> Result<Dto>;
    fn update(&self, data: Dto) -> Result<Dto>;
}

pub struct ListParameters<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

pub struct Client {
    inner: core::workspaces::Client,
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

    fn find(&self, id: &str) -> Result<Option<Dto>> {
        let workspace = find::Operation {
            finder: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(workspace.map(Dto::from_entity))
    }

    fn get(&self, id: &str) -> Result<Dto> {
        let workspace = get::Operation {
            getter: &self.inner,
        }
        .execute(Id::from_str(id)?)?;

        Ok(Dto::from_entity(workspace))
    }

    fn import(&self, data: Dto) -> Result<Dto> {
        let workspace = import::Operation {
            importer: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(Dto::from_entity(workspace))
    }

    fn list(&self, parameters: ListParameters<'_>) -> Result<Vec<Dto>> {
        let ListParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = list::Operation {
            lister: &self.inner,
        }
        .execute(list::Parameters {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces.into_iter().map(Dto::from_entity).collect())
    }

    fn track_access_time(&self, id: &str) -> Result<Dto> {
        use hermione_core::operations::workspaces::get::Get;
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
    pub fn new(connection: Rc<Connection>) -> Self {
        let inner = core::workspaces::Client::new(connection);

        Self { inner }
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

    fn load_entity(self) -> eyre::Result<Entity> {
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
