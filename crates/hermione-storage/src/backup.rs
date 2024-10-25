use crate::database::{CommandRecord, DatabaseProvider, WorkspaceRecord};
use hermione_ops::{
    backup::{Import, Iterate, ListByIds, Update},
    commands::{Command, UpdateCommand},
    workspaces::{UpdateWorkspace, Workspace},
    Result,
};
use rusqlite::{params, types::Value};
use std::{future::Future, rc::Rc};
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: u32 = 100;

pub struct CommandsDatabaseProvider<'a> {
    inner: &'a DatabaseProvider,
}

pub struct WorkspacesDatabaseProvider<'a> {
    inner: &'a DatabaseProvider,
}

impl<'a> CommandsDatabaseProvider<'a> {
    fn connection(&self) -> &rusqlite::Connection {
        self.inner.connection()
    }

    fn insert(&self, record: CommandRecord) -> Result<()> {
        self.inner
            .insert_command(record)
            .map_err(eyre::Error::new)?;

        Ok(())
    }

    fn list_by_page(&self, page_number: u32) -> Result<Vec<Command>> {
        let mut statement = self
            .connection()
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
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                CommandRecord::from_row,
            )
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records
            .into_iter()
            .map(CommandRecord::load_entity)
            .collect();

        Ok(entities)
    }

    pub fn new(inner: &'a DatabaseProvider) -> Self {
        Self { inner }
    }
}

impl<'a> WorkspacesDatabaseProvider<'a> {
    fn connection(&self) -> &rusqlite::Connection {
        self.inner.connection()
    }

    fn insert_workspace(&self, record: WorkspaceRecord) -> Result<()> {
        self.inner
            .insert_workspace(record)
            .map_err(eyre::Error::new)?;

        Ok(())
    }

    pub fn new(inner: &'a DatabaseProvider) -> Self {
        Self { inner }
    }

    fn list_by_page(&self, page_number: u32) -> Result<Vec<Workspace>> {
        let mut statement = self
            .connection()
            .prepare(
                "SELECT
                        id,
                        last_access_time,
                        location,
                        name
                    FROM workspaces
                    ORDER BY name ASC
                    LIMIT ?1 OFFSET ?2",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                |row| row.try_into(),
            )
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}

impl<'a> Import for CommandsDatabaseProvider<'a> {
    type Entity = Command;

    async fn import(&self, command: Command) -> Result<Command> {
        let record = CommandRecord::from_entity(&command)?;
        self.insert(record)?;

        Ok(command)
    }
}

impl<'a> Import for WorkspacesDatabaseProvider<'a> {
    type Entity = Workspace;

    async fn import(&self, entity: Workspace) -> Result<Workspace> {
        let record = WorkspaceRecord::try_from(&entity)?;
        self.insert_workspace(record)?;

        Ok(entity)
    }
}

impl<'a> Iterate for CommandsDatabaseProvider<'a> {
    type Entity = Command;

    async fn iterate<M, MR>(&self, map_fn: M) -> Result<()>
    where
        M: Fn(Vec<Self::Entity>) -> MR,
        MR: Future<Output = Result<()>>,
    {
        let mut page_number = 0;

        loop {
            let commands = self.list_by_page(page_number)?;

            if commands.is_empty() {
                break;
            }

            map_fn(commands).await?;

            page_number += 1;
        }

        Ok(())
    }
}

impl<'a> Iterate for WorkspacesDatabaseProvider<'a> {
    type Entity = Workspace;

    async fn iterate<M, MR>(&self, map_fn: M) -> Result<()>
    where
        M: Fn(Vec<Self::Entity>) -> MR,
        MR: Future<Output = Result<()>>,
    {
        let mut page_number = 0;

        loop {
            let workspaces = self.list_by_page(page_number)?;

            if workspaces.is_empty() {
                break;
            }

            map_fn(workspaces).await?;

            page_number += 1;
        }

        Ok(())
    }
}

impl<'a> ListByIds for CommandsDatabaseProvider<'a> {
    type Entity = Command;

    async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Self::Entity>> {
        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut statement = self
            .connection()
            .prepare(
                "SELECT
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                FROM commands
                WHERE id IN rarray(?1)
                ORDER BY program ASC",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(params![ids], CommandRecord::from_row)
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records
            .into_iter()
            .map(CommandRecord::load_entity)
            .collect();

        Ok(entities)
    }
}

impl<'a> ListByIds for WorkspacesDatabaseProvider<'a> {
    type Entity = Workspace;

    async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Self::Entity>> {
        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut statement = self
            .connection()
            .prepare(
                "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id IN rarray(?1)
                ORDER BY name ASC",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(params![ids], |row| row.try_into())
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}

impl<'a> Update for CommandsDatabaseProvider<'a> {
    type Entity = Command;

    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity> {
        self.inner.update_command(entity)
    }
}

impl<'a> Update for WorkspacesDatabaseProvider<'a> {
    type Entity = Workspace;

    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity> {
        self.inner.update_workspace(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hermione_ops::{
        commands::{CreateCommand, NewCommandParameters},
        workspaces::{CreateWorkspace, NewWorkspaceParameters},
    };
    use std::path::Path;

    const TEST_DB_FILE_PATH: &str = "tests/assets/hermione_test.db3";

    fn prepare_test_db() -> Result<DatabaseProvider> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(eyre::Error::new)?;
        let path = Path::new(&manifest_dir).join(TEST_DB_FILE_PATH);

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        let provider = DatabaseProvider::new(&path).map_err(eyre::Error::new)?;

        Ok(provider)
    }

    #[tokio::test]
    async fn test_list_commands() -> Result<()> {
        let db = prepare_test_db()?;
        let cdb = CommandsDatabaseProvider::new(&db);

        let workspace = db.create_workspace(Workspace::new(NewWorkspaceParameters {
            name: "Test workspace".into(),
            location: None,
        }))?;

        let command1 = db.create_command(Command::new(NewCommandParameters {
            name: "Cmd 1".into(),
            program: "Prg 1".into(),
            workspace_id: workspace.try_id()?,
        }))?;

        let command2 = db.create_command(Command::new(NewCommandParameters {
            name: "Cmd 2".into(),
            program: "Prg 2".into(),
            workspace_id: workspace.try_id()?,
        }))?;

        let commands = cdb.list_by_ids(vec![command2.try_id()?]).await?;

        assert_eq!(commands.len(), 1);

        let command = commands.into_iter().next().unwrap();

        assert_eq!(command, command2);

        let commands = cdb
            .list_by_ids(vec![command2.try_id()?, command1.try_id()?])
            .await?;

        assert_eq!(commands.len(), 2);

        let commands = cdb.list_by_ids(vec![]).await?;

        assert!(commands.is_empty());

        Ok(())
    }
}
