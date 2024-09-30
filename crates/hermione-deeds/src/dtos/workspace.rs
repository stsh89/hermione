use anyhow::Result;
use chrono::{DateTime, Utc};
use hermione_memories::{
    entities::workspace::{Entity, LoadParameters, Location, Name, NewParameters},
    Id,
};
use std::str::FromStr;

pub struct Dto {
    pub id: String,
    pub last_access_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub name: String,
}

impl Dto {
    pub(crate) fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id().map(|id| id.to_string()).unwrap_or_default(),
            last_access_time: entity.last_access_time().map(Into::into),
            location: entity.location().map(ToString::to_string),
            name: entity.name().to_string(),
        }
    }

    pub(crate) fn load_entity(self) -> Result<Entity> {
        Ok(Entity::load(LoadParameters {
            id: Id::from_str(&self.id)?,
            name: Name::new(self.name),
            location: self.location.map(Location::new),
            last_access_time: self.last_access_time.map(From::from),
        }))
    }

    pub(crate) fn new_entity(self) -> Entity {
        Entity::new(NewParameters {
            name: Name::new(self.name),
            location: self.location.map(Location::new),
        })
    }
}
