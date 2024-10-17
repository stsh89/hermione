pub mod commands;

use crate::{records::workspace::Record, Connection, ErrReport};
use chrono::Utc;
use hermione_core::{
    entities::workspace::Entity,
    operations::workspaces::{create, delete, find, get, import, list, track_access_time, update},
    Id, Result,
};
use rusqlite::{params, OptionalExtension, Statement};
use std::rc::Rc;
use uuid::Uuid;

pub struct Client {
    connection: Rc<Connection>,
}

impl Client {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self { connection }
    }

    fn insert(&self, record: Record) -> Result<()> {
        self.connection
            .prepare(
                "INSERT INTO workspaces (
                    id,
                    last_access_time,
                    location,
                    name
                ) VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(ErrReport::err_report)?
            .execute(params![
                record.id,
                record.last_access_time,
                record.location,
                record.name
            ])
            .map_err(ErrReport::err_report)?;

        Ok(())
    }

    fn select_workspace(&self) -> Result<Statement> {
        let statement = self
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

        Ok(statement)
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

impl find::Find for Client {
    fn find(&self, id: Id) -> Result<Option<Entity>> {
        let record = self
            .select_workspace()?
            .query_row([id.as_bytes()], Record::from_row)
            .optional()
            .map_err(ErrReport::err_report)?;

        Ok(record.map(Record::load_entity))
    }
}

impl get::Get for Client {
    fn get(&self, id: Id) -> Result<Entity> {
        let record = self
            .select_workspace()?
            .query_row([id.as_bytes()], Record::from_row)
            .map_err(ErrReport::err_report)?;

        Ok(Record::load_entity(record))
    }
}

impl import::Import for Client {
    fn import(&self, entity: Entity) -> Result<Entity> {
        let record = Record::from_entity(&entity)?;

        self.insert(record)?;

        Ok(entity)
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters {
            name_contains,
            page_size,
            page_number,
        } = parameters;

        let name_contains = format!("%{}%", name_contains.to_lowercase());

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
                ORDER BY last_access_time DESC, name ASC
                LIMIT ?2 OFFSET ?3",
            )
            .map_err(ErrReport::err_report)?;

        let records = statement
            .query_map(
                params![name_contains, page_size, page_number * page_size],
                Record::from_row,
            )
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
