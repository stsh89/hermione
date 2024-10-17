use crate::{records::command::Record, Connection, ErrReport};
use chrono::Utc;
use hermione_core::{
    entities::command::{Entity, ScopedId},
    operations::workspaces::commands::{
        create, delete, find, get, list, track_execution_time, update,
    },
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

    fn select_command(&self) -> Result<Statement> {
        let statement = self
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

        Ok(statement)
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

impl find::Find for Client {
    fn find(&self, id: ScopedId) -> Result<Option<Entity>> {
        let ScopedId { workspace_id, id } = id;

        let record = self
            .select_command()?
            .query_row([id.as_bytes(), workspace_id.as_bytes()], Record::from_row)
            .optional()
            .map_err(ErrReport::err_report)?;

        Ok(record.map(Record::load_entity))
    }
}

impl get::Get for Client {
    fn get(&self, id: ScopedId) -> Result<Entity> {
        let ScopedId { workspace_id, id } = id;

        let record = self
            .select_command()?
            .query_row([id.as_bytes(), workspace_id.as_bytes()], Record::from_row)
            .map_err(ErrReport::err_report)?;

        Ok(Record::load_entity(record))
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let program_contains = format!("%{}%", program_contains.to_lowercase());

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
                ORDER BY last_execute_time DESC, program ASC
                LIMIT ?3 OFFSET ?4",
            )
            .map_err(ErrReport::err_report)?;

        let records = statement
            .query_map(
                params![
                    program_contains,
                    workspace_id.as_bytes(),
                    page_size,
                    page_number * page_size
                ],
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
