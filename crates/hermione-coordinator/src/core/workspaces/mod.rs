pub mod commands;

use chrono::{DateTime, Utc};
use hermione_core::{
    entities::workspace::{Entity, LoadParameters, Location, Name},
    operations::workspaces::{create, delete, get::{self, Get}, list, track_access_time, update},
    Id, Result,
};
use hermione_json::collections::Client as InnerClient;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_access_time: Option<DateTime<Utc>>,

    location: String,
    name: String,
}

pub struct Client {
    connection: Connection,
}

impl Client {
    pub fn new(path: PathBuf) -> eyre::Result<Self> {
        let connection = Connection::open(path)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id BLOB PRIMARY KEY,
                last_access_time INTEGER,
                location TEXT NOT NULL,
                name TEXT NOT NULL
            )",
            (),
        )?;

        Ok(Self { connection })
    }

    // pub fn list(&self) -> eyre::Result<Vec<Record>> {
    //     let mut stmt = self.connection.prepare(
    //         "SELECT
    //             id,
    //             last_access_time,
    //             location,
    //             name
    //         FROM workspaces
    //         ORDER BY name ASC, last_access_time DESC",
    //     )?;

    //     let mapped_rows = stmt.query_map([], |row| {
    //         Ok(Record {
    //             id: Uuid::from_bytes(row.get(0)?),
    //             last_access_time: row
    //                 .get::<_, Option<i64>>(1)?
    //                 .map(chrono::DateTime::from_timestamp_nanos),
    //             location: row.get(2)?,
    //             name: row.get(3)?,
    //         })
    //     })?;

    //     let records = mapped_rows.collect::<eyre::Result<Vec<_>, _>>()?;

    //     // let mut records = Vec::with_capacity(mapped_rows.);

    //     // for row in mapped_rows {
    //     //     records.push(row?);
    //     // }

    //     Ok(records)
    // }

    pub fn insert(&self, record: Record) -> eyre::Result<()> {
        self.connection.execute(
            "INSERT INTO workspaces (id, last_access_time, location, name) VALUES (?1, ?2, ?3, ?4)",
            (
                record.id.as_bytes(),
                record.last_access_time.map(|t| t.timestamp_nanos_opt()),
                record.location,
                record.name
            ),
        )?;

        Ok(())
    }
}

impl create::Create for Client {
    fn create(&self, mut entity: Entity) -> Result<Entity> {
        let id = Uuid::new_v4();
        entity.set_id(Id::new(id))?;

        let record = Record::from_entity(&entity)?;

        self.insert(record)?;

        Ok(entity)
    }
}

impl delete::Delete for Client {
    fn delete(&self, id: Id) -> Result<()> {
        // let mut record = self.list()?;

        // record.retain(|record| record.id != *id);

        // self.insert(record)?;

        // Ok(())

        todo!()
    }
}

impl get::Get for Client {
    fn get(&self, id: Id) -> Result<Entity> {
        // let records = self.list()?;

        // let record = records
        //     .into_iter()
        //     .find(|record| record.id == *id)
        //     .ok_or_else(|| eyre::eyre!("Workspace with id {} not found", id))?;

        // let entity = record.load_entity();

        // Ok(entity)

        let mut stmt = self.connection.prepare(
            "SELECT
                id,
                last_access_time,
                location,
                name
            FROM workspaces
            WHERE id = :id",
        ).map_err(eyre::Error::new)?;

        let mapped_rows = stmt.query_map(&[(":id", &id.as_bytes())], |row| {
            Ok(Record {
                id: Uuid::from_bytes(row.get(0)?),
                last_access_time: row
                    .get::<_, Option<i64>>(1)?
                    .map(chrono::DateTime::from_timestamp_nanos),
                location: row.get(2)?,
                name: row.get(3)?,
            })
        }).map_err(eyre::Error::new)?;

        let records = mapped_rows.collect::<std::result::Result<Vec<_>, _>>().map_err(eyre::Error::new)?;

        let mut entities: Vec<Entity> = records.into_iter().map(Record::load_entity).collect();

        if let Some(entity) = entities.pop() {
            return Ok(entity);
        }

        Err(eyre::Error::msg("More than one workspace with such id").into())
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters { name_contains } = parameters;
        // let mut records = self.list()?;

        // if let Some(query) = name_contains.map(|q| q.to_lowercase()) {
        //     records.retain(|w| w.name.to_lowercase().contains(&query));
        // }

        // records.sort_unstable_by(|a, b| a.last_access_time.cmp(&b.last_access_time).reverse());

        // let entities = records.into_iter().map(Record::load_entity).collect();

        // Ok(entities)

        let mut stmt = self.connection.prepare(
            "SELECT
                id,
                last_access_time,
                location,
                name
            FROM workspaces
            WHERE LOWER(name) LIKE :name_like
            ORDER BY last_access_time DESC, name ASC",
        ).map_err(eyre::Error::new)?;

        let q = name_contains.unwrap_or_default();
        let name_like = format!("%{}%", q.to_lowercase());
        let mapped_rows = stmt.query_map(&[(":name_like", &name_like)], |row| {
            Ok(Record {
                id: Uuid::from_bytes(row.get(0)?),
                last_access_time: row
                    .get::<_, Option<i64>>(1)?
                    .map(chrono::DateTime::from_timestamp_nanos),
                location: row.get(2)?,
                name: row.get(3)?,
            })
        }).map_err(eyre::Error::new)?;

        let records = mapped_rows.collect::<std::result::Result<Vec<_>, _>>().map_err(eyre::Error::new)?;

        let entities = records.into_iter().map(Record::load_entity).collect();
        // let mut records = Vec::with_capacity(mapped_rows.);

        // for row in mapped_rows {
        //     records.push(row?);
        // }

        Ok(entities)
    }
}

impl track_access_time::Track for Client {
    fn track(&self, entity: Entity) -> Result<Entity> {
        let Some(id) = entity.id() else {
            return Err(
                eyre::eyre!("Attempt to track access time for workspace without id").into(),
            );
        };

        let mut stmt = self.connection.prepare(
            "UPDATE workspaces
            SET last_access_time = ?1
            WHERE id = ?2",
        ).map_err(eyre::Error::new)?;

        let last_access_time = Utc::now().timestamp_nanos_opt().ok_or(eyre::eyre!("Failed to get timestamp"))?;
        stmt.execute(params![last_access_time, id.as_bytes()]).map_err(eyre::Error::new)?;

        // let mut records = self.list()?;
        // let record = records
        //     .iter_mut()
        //     .find(|record| record.id == *id)
        //     .ok_or(eyre::eyre!("Workspace with id {} not found", id,))?;

        // record.last_access_time = Some(Utc::now());

        // self.insert(records)?;

        // use get::Get;
        // self.get(id)

        self.get(id)
    }
}

impl update::Update for Client {
    fn update(&self, entity: Entity) -> Result<Entity> {
        // let Some(id) = entity.id() else {
        //     return Err(eyre::eyre!("Attemp to update workspace without id").into());
        // };

        // let mut records = self.list()?;
        // let record = records
        //     .iter_mut()
        //     .find(|record| record.id == *id)
        //     .ok_or_else(|| eyre::eyre!("Workspace with id {}not found", id))?;

        // record.name = entity.name().to_string();
        // record.location = entity
        //     .location()
        //     .map(ToString::to_string)
        //     .unwrap_or_default();

        // self.insert(records)?;

        // Ok(entity)

        todo!()
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self> {
        Ok(Self {
            id: *entity.id().ok_or(eyre::eyre!("Record without id"))?,
            last_access_time: entity.last_access_time().map(From::from),
            location: entity
                .location()
                .map(ToString::to_string)
                .unwrap_or_default(),
            name: entity.name().to_string(),
        })
    }

    fn load_entity(self) -> Entity {
        Entity::load(LoadParameters {
            id: Id::new(self.id),
            last_access_time: self.last_access_time.map(From::from),
            location: Some(Location::new(self.location)),
            name: Name::new(self.name),
        })
    }
}
