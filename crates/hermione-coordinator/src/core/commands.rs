use crate::{records::command::Record, Connection, ErrReport};
use hermione_core::{entities::command::Entity, operations::commands::list, Result};
use rusqlite::params;
use std::rc::Rc;

pub struct Client {
    connection: Rc<Connection>,
}

impl Client {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self { connection }
    }
}

impl list::List for Client {
    fn list(&self, parameters: list::ListParameters) -> Result<Vec<Entity>> {
        let list::ListParameters {
            page_number,
            page_size,
        } = parameters;

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
                ORDER BY program ASC
                LIMIT ?1 OFFSET ?2",
            )
            .map_err(ErrReport::err_report)?;

        let records = statement
            .query_map(
                params![page_size, page_number * page_size],
                Record::from_row,
            )
            .map_err(ErrReport::err_report)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(ErrReport::err_report)?;

        let entities = records.into_iter().map(Record::load_entity).collect();

        Ok(entities)
    }
}
