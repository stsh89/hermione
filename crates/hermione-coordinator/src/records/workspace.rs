use chrono::{DateTime, Utc};
use hermione_core::{
    entities::workspace::{Entity, LoadParameters, Location, Name},
    Id, Result,
};
use uuid::{Bytes, Uuid};

pub struct Record {
    pub id: Bytes,
    pub last_access_time: Option<i64>,
    pub location: Option<String>,
    pub name: String,
}

impl Record {
    pub fn from_entity(entity: &Entity) -> Result<Self> {
        let id = *entity
            .id()
            .ok_or(eyre::eyre!("Record without id"))?
            .as_bytes();

        let last_access_time = entity
            .last_access_time()
            .and_then(|date_time| Into::<DateTime<Utc>>::into(date_time).timestamp_nanos_opt());

        Ok(Self {
            id,
            last_access_time,
            location: entity.location().map(ToString::to_string),
            name: entity.name().to_string(),
        })
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Record {
            id: row.get(0)?,
            last_access_time: row.get(1)?,
            location: row.get(2)?,
            name: row.get(3)?,
        })
    }

    pub fn load_entity(self) -> Entity {
        let id = Id::new(Uuid::from_bytes(self.id));

        let last_access_time = self
            .last_access_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        Entity::load(LoadParameters {
            id,
            last_access_time,
            location: self.location.map(Location::new),
            name: Name::new(self.name),
        })
    }
}
