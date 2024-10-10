use chrono::{DateTime, Utc};
use hermione_core::{
    entities::command::{Entity, LoadParameters, Name, Program, ScopedId},
    operations::workspaces::commands::{create, delete, get, list, track_execution_time, update},
    Id, Result,
};
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::{Bytes, Uuid};

use crate::ErrReport;

struct Record {
    id: Bytes,
    last_execute_time: Option<i64>,
    name: String,
    program: String,
    workspace_id: Bytes,
}

pub struct Client {
    connection: Connection,
}

impl Client {
    pub fn new(connection: Connection) -> eyre::Result<Self> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id BLOB PRIMARY KEY,
                last_execute_time INTEGER,
                name TEXT NOT NULL,
                program TEXT NOT NULL,
                workspace_id BLOB NOT NULL
            )",
            (),
        )?;

        connection.execute(
            "CREATE INDEX IF NOT EXISTS
            commands_workspace_id_idx
            ON commands(workspace_id)",
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
                "INSERT INTO commands (
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                ) VALUES (?1, ?2, ?3, ?4, ?5)",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![
                record.id,
                record.last_execute_time,
                record.name,
                record.program,
                record.workspace_id
            ])
            .map_err(ErrReport::err_report)?;

        Ok(entity)
    }
}

impl delete::Delete for Client {
    fn delete(&self, id: ScopedId) -> Result<()> {
        let ScopedId { workspace_id, id } = id;

        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE id = ?1 AND workspace_id = ?2")
            .map_err(ErrReport::err_report)?;

        statement
            .execute([id.as_bytes(), workspace_id.as_bytes()])
            .map_err(ErrReport::err_report)?;

        Ok(())
    }
}

impl get::Get for Client {
    fn get(&self, id: ScopedId) -> Result<Entity> {
        let ScopedId { workspace_id, id } = id;

        let mut statement = self
            .connection
            .prepare(
                "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE id = ?1 AND workspace_id = ?2",
            )
            .map_err(ErrReport::err_report)?;

        let record = statement
            .query_row([id.as_bytes(), workspace_id.as_bytes()], Record::from_row)
            .map_err(ErrReport::err_report)?;

        Ok(Record::load_entity(record))
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters {
            workspace_id,
            program_contains,
        } = parameters;

        let program_contains = program_contains
            .map(|q| format!("%{}%", q.to_lowercase()))
            .unwrap_or("%%".into());

        let mut statement = self
            .connection
            .prepare(
                "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE LOWER(program) LIKE ?1 AND workspace_id = ?2
            ORDER BY last_execute_time DESC, program ASC",
            )
            .map_err(ErrReport::err_report)?;

        let records = statement
            .query_map(
                params![program_contains, workspace_id.as_bytes()],
                Record::from_row,
            )
            .map_err(ErrReport::err_report)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(ErrReport::err_report)?;

        let entities = records.into_iter().map(Record::load_entity).collect();

        Ok(entities)
    }
}

impl track_execution_time::Track for Client {
    fn track(&self, entity: Entity) -> Result<Entity> {
        let record = Record::from_entity(&entity)?;

        let last_execute_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE commands
                SET last_execute_time = ?1
                WHERE id = ?2 AND workspace_id = ?3",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![last_execute_time, record.id, record.workspace_id])
            .map_err(ErrReport::err_report)?;

        use get::Get;
        self.get(ScopedId {
            id: Id::new(Uuid::from_bytes(record.id)),
            workspace_id: Id::new(Uuid::from_bytes(record.workspace_id)),
        })
    }
}

impl update::Update for Client {
    fn update(&self, entity: Entity) -> Result<Entity> {
        let record = Record::from_entity(&entity)?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE commands
                SET
                    name = ?1,
                    program = ?2
                WHERE id = ?3 AND workspace_id = ?4",
            )
            .map_err(ErrReport::err_report)?;

        statement
            .execute(params![
                record.name,
                record.program,
                record.id,
                record.workspace_id
            ])
            .map_err(ErrReport::err_report)?;

        use get::Get;
        self.get(ScopedId {
            id: Id::new(Uuid::from_bytes(record.id)),
            workspace_id: Id::new(Uuid::from_bytes(record.workspace_id)),
        })
    }
}

impl Record {
    fn from_entity(entity: &Entity) -> Result<Self> {
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

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Record {
            id: row.get(0)?,
            last_execute_time: row.get(1)?,
            name: row.get(2)?,
            program: row.get(3)?,
            workspace_id: row.get(4)?,
        })
    }

    fn load_entity(self) -> Entity {
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
