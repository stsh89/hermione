pub mod commands;

use crate::ErrReport;
use chrono::{DateTime, Utc};
use hermione_core::{
    entities::workspace::{Entity, LoadParameters, Location, Name},
    operations::workspaces::{create, delete, get, list, track_access_time, update},
    Id, Result,
};
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::{Bytes, Uuid};

struct Record {
    id: Bytes,
    last_access_time: Option<i64>,
    location: Option<String>,
    name: String,
}

pub struct Client {
    connection: Connection,
}

impl Client {
    pub fn new(connection: Connection) -> eyre::Result<Self> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id BLOB PRIMARY KEY,
                last_access_time INTEGER,
                location TEXT,
                name TEXT NOT NULL
            )",
            (),
        )?;

        Ok(Self { connection })
    }
}

impl create::Create for Client {
    fn create(&self, mut entity: Entity) -> Result<Entity> {
        let id = Uuid::new_v4();
        entity.set_id(Id::new(id))?;

        let record = Record::from_entity(&entity)?;

        let mut statement = self
            .connection
            .prepare(
                "INSERT INTO workspaces (
                    id,
                    last_access_time,
                    location,
                    name
                ) VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![
                record.id,
                record.last_access_time,
                record.location,
                record.name
            ])
            .map_err(ErrReport::err_report)?;

        Ok(entity)
    }
}

impl delete::Delete for Client {
    fn delete(&self, id: Id) -> Result<()> {
        // TODO: apply transaction

        let mut statement = self
            .connection
            .prepare("DELETE FROM workspaces WHERE id = ?1")
            .map_err(ErrReport::err_report)?;

        statement
            .execute([id.as_bytes()])
            .map_err(ErrReport::err_report)?;

        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE workspace_id = ?1")
            .map_err(ErrReport::err_report)?;

        statement
            .execute([id.as_bytes()])
            .map_err(ErrReport::err_report)?;

        Ok(())
    }
}

impl get::Get for Client {
    fn get(&self, id: Id) -> Result<Entity> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id = ?1",
            )
            .map_err(ErrReport::err_report)?;

        let record = statement
            .query_row([id.as_bytes()], Record::from_row)
            .map_err(ErrReport::err_report)?;

        Ok(Record::load_entity(record))
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters { name_contains } = parameters;

        let name_contains = name_contains
            .map(|q| format!("%{}%", q.to_lowercase()))
            .unwrap_or("%%".into());

        let mut statement = self
            .connection
            .prepare(
                "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE LOWER(name) LIKE ?1
                ORDER BY last_access_time DESC, name ASC",
            )
            .map_err(ErrReport::err_report)?;

        let records = statement
            .query_map([name_contains], Record::from_row)
            .map_err(ErrReport::err_report)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(ErrReport::err_report)?;

        let entities = records.into_iter().map(Record::load_entity).collect();

        Ok(entities)
    }
}

impl track_access_time::Track for Client {
    fn track(&self, entity: Entity) -> Result<Entity> {
        let record = Record::from_entity(&entity)?;

        let last_access_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE workspaces
                SET last_access_time = ?1
                WHERE id = ?2",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![last_access_time, record.id])
            .map_err(ErrReport::err_report)?;

        use get::Get;
        self.get(Id::new(Uuid::from_bytes(record.id)))
    }
}

impl update::Update for Client {
    fn update(&self, entity: Entity) -> Result<Entity> {
        let record = Record::from_entity(&entity)?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE workspaces
                SET
                    location = ?1,
                    name = ?2
                WHERE id = ?3",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![record.location, record.name, record.id])
            .map_err(ErrReport::err_report)?;

        use get::Get;
        self.get(Id::new(Uuid::from_bytes(record.id)))
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self> {
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

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Record {
            id: row.get(0)?,
            last_access_time: row.get(1)?,
            location: row.get(2)?,
            name: row.get(3)?,
        })
    }

    fn load_entity(self) -> Entity {
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
