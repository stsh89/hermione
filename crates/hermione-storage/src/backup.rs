use crate::sqlite::{CommandRecord, Error, SqliteProvider, WorkspaceRecord};
use hermione_ops::{
    backup::{Import, Iterate, ListByIds, Update},
    commands::{Command, UpdateCommand},
    workspaces::{UpdateWorkspace, Workspace},
    Result,
};
use rusqlite::{params, types::Value};
use std::{
    future::{self, Future},
    rc::Rc,
    sync::RwLock,
};
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: u32 = 100;

pub struct SqliteCommandsProvider<'a> {
    inner: &'a SqliteProvider,
}

pub struct SqliteCommandsIteratorProvider<'a> {
    inner: &'a SqliteProvider,
    page_number: RwLock<u32>,
}

pub struct SqliteWorkspacesProvider<'a> {
    inner: &'a SqliteProvider,
}

pub struct SqliteWorkspacesIteratorProvider<'a> {
    inner: &'a SqliteProvider,
    page_number: RwLock<u32>,
}

impl<'a> SqliteCommandsProvider<'a> {
    fn connection(&self) -> &rusqlite::Connection {
        self.inner.connection()
    }

    fn import_command(&self, command: Command) -> Result<Command> {
        let record = CommandRecord::from_entity(&command)?;

        self.inner
            .insert_command(record)
            .map_err(Into::<Error>::into)?;

        Ok(command)
    }

    fn list_commands_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Command>> {
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
            .map_err(Into::<Error>::into)?;

        let rows = statement
            .query_map(params![ids], CommandRecord::from_row)
            .map_err(Into::<Error>::into)?;

        let records = rows
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::<Error>::into)?;

        let entities = records
            .into_iter()
            .map(CommandRecord::load_entity)
            .collect();

        Ok(entities)
    }

    pub fn new(inner: &'a SqliteProvider) -> Self {
        Self { inner }
    }
}

impl<'a> SqliteCommandsIteratorProvider<'a> {
    fn list_by_page(&self, page_number: u32) -> Result<Vec<Command>> {
        let mut statement = self
            .inner
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
            .map_err(Into::<Error>::into)?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                CommandRecord::from_row,
            )
            .map_err(Into::<Error>::into)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(Into::<Error>::into)?;

        let entities = records
            .into_iter()
            .map(CommandRecord::load_entity)
            .collect();

        Ok(entities)
    }

    fn next_commands(&self) -> Result<Option<Vec<Command>>> {
        let mut page_number = self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))?;

        let commands = self.list_by_page(*page_number)?;

        if commands.is_empty() {
            return Ok(None);
        }

        *page_number += 1;

        Ok(Some(commands))
    }

    pub fn new(inner: &'a SqliteProvider) -> Self {
        Self {
            inner,
            page_number: RwLock::new(0),
        }
    }
}

impl<'a> SqliteWorkspacesProvider<'a> {
    fn import_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let record = WorkspaceRecord::try_from(&workspace)?;

        self.inner
            .insert_workspace(record)
            .map_err(Into::<Error>::into)?;

        Ok(workspace)
    }

    fn list_workspaces_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Workspace>> {
        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut statement = self
            .inner
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
            .map_err(Into::<Error>::into)?;

        let rows = statement
            .query_map(params![ids], |row| row.try_into())
            .map_err(Into::<Error>::into)?;
        let records = rows
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()
            .map_err(Into::<Error>::into)?;
        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }

    pub fn new(inner: &'a SqliteProvider) -> Self {
        Self { inner }
    }
}

impl<'a> SqliteWorkspacesIteratorProvider<'a> {
    fn list_by_page(&self, page_number: u32) -> Result<Vec<Workspace>> {
        let mut statement = self
            .inner
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
            .map_err(Into::<Error>::into)?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                |row| row.try_into(),
            )
            .map_err(Into::<Error>::into)?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()
            .map_err(Into::<Error>::into)?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }

    fn next_workspaces(&self) -> Result<Option<Vec<Workspace>>> {
        let mut page_number = self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))?;

        let workspaces = self.list_by_page(*page_number)?;

        if workspaces.is_empty() {
            return Ok(None);
        }

        *page_number += 1;

        Ok(Some(workspaces))
    }

    pub fn new(inner: &'a SqliteProvider) -> Self {
        Self {
            inner,
            page_number: RwLock::new(0),
        }
    }
}

impl<'a> Import for SqliteCommandsProvider<'a> {
    type Entity = Command;

    fn import(&self, command: Command) -> impl Future<Output = Result<Command>> {
        let result = self.import_command(command);

        future::ready(result)
    }
}

impl<'a> Import for SqliteWorkspacesProvider<'a> {
    type Entity = Workspace;

    fn import(&self, workspace: Workspace) -> impl Future<Output = Result<Workspace>> {
        let result = self.import_workspace(workspace);

        future::ready(result)
    }
}

impl<'a> Iterate for SqliteCommandsIteratorProvider<'a> {
    type Entity = Command;

    fn iterate(&self) -> impl Future<Output = Result<Option<Vec<Self::Entity>>>> {
        let result = self.next_commands();

        future::ready(result)
    }
}

impl<'a> Iterate for SqliteWorkspacesIteratorProvider<'a> {
    type Entity = Workspace;

    fn iterate(&self) -> impl Future<Output = Result<Option<Vec<Self::Entity>>>> {
        let result = self.next_workspaces();

        future::ready(result)
    }
}

impl<'a> ListByIds for SqliteCommandsProvider<'a> {
    type Entity = Command;

    fn list_by_ids(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Self::Entity>>> {
        let result = self.list_commands_by_ids(ids);

        future::ready(result)
    }
}

impl<'a> ListByIds for SqliteWorkspacesProvider<'a> {
    type Entity = Workspace;

    fn list_by_ids(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Self::Entity>>> {
        let result = self.list_workspaces_by_ids(ids);

        future::ready(result)
    }
}

impl<'a> Update for SqliteCommandsProvider<'a> {
    type Entity = Command;

    fn update(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>> {
        let result = self.inner.update_command(entity);

        future::ready(result)
    }
}

impl<'a> Update for SqliteWorkspacesProvider<'a> {
    type Entity = Workspace;

    fn update(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>> {
        let result = self.inner.update_workspace(entity);

        future::ready(result)
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

    fn prepare_test_db() -> Result<SqliteProvider> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(eyre::Error::new)?;
        let path = Path::new(&manifest_dir).join(TEST_DB_FILE_PATH);

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        let provider = SqliteProvider::new(&path).map_err(eyre::Error::new)?;

        Ok(provider)
    }

    #[tokio::test]
    async fn test_list_commands() -> Result<()> {
        let db = prepare_test_db()?;
        let cdb = SqliteCommandsProvider::new(&db);

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
