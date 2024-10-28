use crate::{CommandRecord, StorageProvider, StorageProviderResult, WorkspaceRecord};
use hermione_ops::{
    backup::{
        BckImportCommand, BckImportWorkspace, BckIterateCommands, BckIterateWorkspaces,
        BckListCommands, BckListWorkspaces, BckUpdateCommand, BckUpdateWorkspace,
    },
    commands::{Command, UpdateCommand},
    workspaces::{UpdateWorkspace, Workspace},
    Result,
};
use rusqlite::{params, types::Value, Connection};
use std::{
    future::{self, Future},
    rc::Rc,
    sync::RwLock,
};
use uuid::Uuid;

const DEFAULT_PAGE_SIZE: u32 = 100;

impl<'a> StorageProvider<'a> {
    pub fn commands_iterator(&self) -> CommandsIterator<'a> {
        CommandsIterator {
            connection: self.connection,
            page_number: RwLock::new(0),
        }
    }

    fn list_commands_by_ids(&self, ids: Vec<Uuid>) -> StorageProviderResult<Vec<CommandRecord>> {
        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut statement = self.connection.prepare(
            "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE id IN rarray(?1)
            ORDER BY program ASC",
        )?;

        let rows = statement.query_map(params![ids], |row| row.try_into())?;

        let records = rows.collect::<std::result::Result<Vec<CommandRecord>, _>>()?;

        Ok(records)
    }

    fn list_workspaces_by_ids(
        &self,
        ids: Vec<Uuid>,
    ) -> StorageProviderResult<Vec<WorkspaceRecord>> {
        let ids: Vec<Vec<u8>> = ids.into_iter().map(|id| id.into_bytes().to_vec()).collect();
        let ids = Rc::new(ids.into_iter().map(Value::from).collect::<Vec<Value>>());

        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut statement = self.connection.prepare(
            "SELECT
                id,
                last_access_time,
                location,
                name
            FROM workspaces
            WHERE id IN rarray(?1)
            ORDER BY name ASC",
        )?;

        let rows = statement.query_map(params![ids], |row| row.try_into())?;

        let records = rows.collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()?;

        Ok(records)
    }

    pub fn workspaces_iterator(&self) -> WorkspacesIterator<'a> {
        WorkspacesIterator {
            connection: self.connection,
            page_number: RwLock::new(0),
        }
    }
}

pub struct CommandsIterator<'a> {
    connection: &'a Connection,
    page_number: RwLock<u32>,
}

pub struct WorkspacesIterator<'a> {
    connection: &'a Connection,
    page_number: RwLock<u32>,
}

impl<'a> CommandsIterator<'a> {
    fn list_commands_by_page(&self, page_number: u32) -> StorageProviderResult<Vec<CommandRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            ORDER BY program ASC
            LIMIT ?1 OFFSET ?2",
        )?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                |row| row.try_into(),
            )?
            .collect::<std::result::Result<Vec<CommandRecord>, _>>()?;

        Ok(records)
    }
}

impl<'a> WorkspacesIterator<'a> {
    fn list_workspaces_by_page(
        &self,
        page_number: u32,
    ) -> StorageProviderResult<Vec<WorkspaceRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT
                id,
                last_access_time,
                location,
                name
            FROM workspaces
            ORDER BY name ASC
            LIMIT ?1 OFFSET ?2",
        )?;

        let records = statement
            .query_map(
                params![DEFAULT_PAGE_SIZE, page_number * DEFAULT_PAGE_SIZE],
                |row| row.try_into(),
            )?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()?;

        Ok(records)
    }
}

impl<'a> BckImportCommand for StorageProvider<'a> {
    fn bck_import_command(&self, command: Command) -> impl Future<Output = Result<Command>> {
        let record = match CommandRecord::try_from(&command) {
            Ok(record) => record,
            Err(error) => return future::ready(Err(error)),
        };

        if let Err(err) = self.insert_command(record) {
            return future::ready(Err(err.into()));
        }

        future::ready(Ok(command))
    }
}

impl<'a> BckImportWorkspace for StorageProvider<'a> {
    fn bck_import_workspace(
        &self,
        workspace: Workspace,
    ) -> impl Future<Output = Result<Workspace>> {
        let record = match WorkspaceRecord::try_from(&workspace) {
            Ok(record) => record,
            Err(error) => return future::ready(Err(error)),
        };

        if let Err(err) = self.insert_workspace(record) {
            return future::ready(Err(err.into()));
        }

        future::ready(Ok(workspace))
    }
}

impl<'a> BckIterateCommands for CommandsIterator<'a> {
    fn bck_iterate_commands(&self) -> impl Future<Output = Result<Option<Vec<Command>>>> {
        let mut page_number = match self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))
        {
            Ok(page_number) => page_number,
            Err(err) => return future::ready(Err(err.into())),
        };

        let records = match self.list_commands_by_page(*page_number) {
            Ok(records) => records,
            Err(err) => return future::ready(Err(err.into())),
        };

        if records.is_empty() {
            return future::ready(Ok(None));
        }

        *page_number += 1;

        let commands = records.into_iter().map(Into::into).collect();

        future::ready(Ok(Some(commands)))
    }
}

impl<'a> BckIterateWorkspaces for WorkspacesIterator<'a> {
    fn bck_iterate_workspaces(&self) -> impl Future<Output = Result<Option<Vec<Workspace>>>> {
        let mut page_number = match self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))
        {
            Ok(page_number) => page_number,
            Err(err) => return future::ready(Err(err.into())),
        };

        let records = match self.list_workspaces_by_page(*page_number) {
            Ok(records) => records,
            Err(err) => return future::ready(Err(err.into())),
        };

        if records.is_empty() {
            return future::ready(Ok(None));
        }

        *page_number += 1;

        let workspaces = records.into_iter().map(Into::into).collect();

        future::ready(Ok(Some(workspaces)))
    }
}

impl<'a> BckListCommands for StorageProvider<'a> {
    fn bck_list_commands(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Command>>> {
        let result = self.list_commands_by_ids(ids);

        let result = result
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(Into::into);

        future::ready(result)
    }
}
impl<'a> BckListWorkspaces for StorageProvider<'a> {
    fn bck_list_workspaces(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Workspace>>> {
        let result = self.list_workspaces_by_ids(ids);

        let result = result
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(Into::into);

        future::ready(result)
    }
}

impl<'a> BckUpdateCommand for StorageProvider<'a> {
    fn bck_update_command(&self, entity: Command) -> impl Future<Output = Result<Command>> {
        let result = UpdateCommand::update_command(self, entity);

        future::ready(result)
    }
}

impl<'a> BckUpdateWorkspace for StorageProvider<'a> {
    fn bck_update_workspace(&self, entity: Workspace) -> impl Future<Output = Result<Workspace>> {
        let result = UpdateWorkspace::update_workspace(self, entity);

        future::ready(result)
    }
}
