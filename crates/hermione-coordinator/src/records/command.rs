use chrono::{DateTime, Utc};
use hermione_core::{
    entities::command::{Entity, LoadParameters, Name, Program},
    Id, Result,
};
use uuid::{Bytes, Uuid};

pub struct Record {
    pub id: Bytes,
    pub last_execute_time: Option<i64>,
    pub name: String,
    pub program: String,
    pub workspace_id: Bytes,
}

impl Record {
    pub fn from_entity(entity: &Entity) -> Result<Self> {
        let id = *entity
            .id()
            .ok_or(eyre::eyre!("Record without id"))?
            .as_bytes();

        let last_execute_time = entity
            .last_execute_time()
            .and_then(|date_time| Into::<DateTime<Utc>>::into(date_time).timestamp_nanos_opt());

        Ok(Self {
            id,
            last_execute_time,
            name: entity.name().to_string(),
            program: entity.program().to_string(),
            workspace_id: *entity.workspace_id().as_bytes(),
        })
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Record {
            id: row.get(0)?,
            last_execute_time: row.get(1)?,
            name: row.get(2)?,
            program: row.get(3)?,
            workspace_id: row.get(4)?,
        })
    }

    pub fn load_entity(self) -> Entity {
        let id = Id::new(Uuid::from_bytes(self.id));

        let last_execute_time = self
            .last_execute_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        let workspace_id = Id::new(Uuid::from_bytes(self.workspace_id));

        Entity::load(LoadParameters {
            id,
            last_execute_time,
            name: Name::new(self.name),
            program: Program::new(self.program),
            workspace_id,
        })
    }
}
