#[cfg(feature = "extensions")]
use chrono::Utc;

#[cfg(feature = "extensions")]
use hermione_ops::extensions::{TrackCommandExecutionTime, TrackWorkspaceAccessTime};

use chrono::DateTime;
use hermione_ops::{
    backup::{Import, Iterate, ListByIds, Update},
    commands::{
        Command, CommandWorkspaceScopedId, CreateCommand, DeleteCommandFromWorkspace,
        GetCommandFromWorkspace, ListCommandsWithinWorkspace,
        ListCommandsWithinWorkspaceParameters, LoadCommandParameters, UpdateCommand,
    },
    workspaces::{
        CreateWorkspace, DeleteWorkspace, GetWorkspace, ListWorkspaces, ListWorkspacesParameters,
        LoadWorkspaceParameters, UpdateWorkspace, Workspace,
    },
    Error, Result,
};
use rusqlite::{params, types::Value, Connection};
use std::{
    fs,
    future::{self, Future},
    io,
    path::{Path, PathBuf},
    process,
    rc::Rc,
    str,
    sync::RwLock,
};
use uuid::{Bytes, Uuid};

const DEFAULT_PAGE_SIZE: u32 = 100;

pub type StorageProviderResult<T> = std::result::Result<T, StorageProviderError>;

struct CommandRecord {
    id: Bytes,
    last_execute_time: Option<i64>,
    name: String,
    program: String,
    workspace_id: Bytes,
}

#[cfg(feature = "backup")]
pub struct BackupProvider<'a, T> {
    connection: &'a Connection,
    phantom: std::marker::PhantomData<T>,
    page_number: RwLock<u32>,
}

struct ListCommandsWithinWorkspaceInput<'a> {
    program_contains: &'a str,
    workspace_id: Uuid,
    page_number: u32,
    page_size: u32,
}

struct ListWorkspacesInput<'a> {
    name_contains: &'a str,
    page_size: u32,
    page_number: u32,
}

pub struct StorageProvider<'a> {
    connection: &'a Connection,
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct StorageProviderError(#[from] rusqlite::Error);

struct WorkspaceRecord {
    id: Bytes,
    last_access_time: Option<i64>,
    location: Option<String>,
    name: String,
}

#[cfg(feature = "backup")]
impl<'a, T> BackupProvider<'a, T> {
    fn storage_provider(&self) -> StorageProvider<'a> {
        StorageProvider {
            connection: self.connection,
        }
    }
}

impl<'a> StorageProvider<'a> {
    pub fn connect(folder_path: &Path) -> StorageProviderResult<Connection> {
        let connection = Connection::open(folder_path.join("hermione.db3"))?;

        Ok(connection)
    }

    #[cfg(feature = "backup")]
    pub fn commands_backup_provider(&self) -> BackupProvider<'a, Command> {
        self.backup_provider()
    }

    #[cfg(feature = "backup")]
    pub fn workspaces_backup_provider(&self) -> BackupProvider<'a, Workspace> {
        self.backup_provider()
    }

    #[cfg(feature = "backup")]
    fn backup_provider<T>(&self) -> BackupProvider<'a, T> {
        BackupProvider {
            connection: self.connection,
            phantom: std::marker::PhantomData,
            page_number: RwLock::new(0),
        }
    }

    fn delete_command_from_workspace(
        &self,
        workspace_id: Uuid,
        command_id: Uuid,
    ) -> StorageProviderResult<()> {
        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE id = ?1 AND workspace_id = ?2")?;

        statement.execute([command_id.as_bytes(), workspace_id.as_bytes()])?;

        Ok(())
    }

    fn delete_workspace(&self, id: Uuid) -> StorageProviderResult<()> {
        let mut statement = self
            .connection
            .prepare("DELETE FROM workspaces WHERE id = ?1")?;

        statement.execute([id.as_bytes()])?;

        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE workspace_id = ?1")?;

        statement.execute([id.as_bytes()])?;

        Ok(())
    }

    fn get_command_from_workspace(
        &self,
        workspace_id: Uuid,
        command_id: Uuid,
    ) -> StorageProviderResult<CommandRecord> {
        let mut statement = self.connection.prepare(
            "SELECT
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                FROM commands
                WHERE id = ?1 AND workspace_id = ?2",
        )?;

        let record = statement
            .query_row([command_id.as_bytes(), workspace_id.as_bytes()], |row| {
                row.try_into()
            })?;

        Ok(record)
    }

    fn get_workspace(&self, id: Uuid) -> StorageProviderResult<WorkspaceRecord> {
        let workspace = self
            .connection
            .prepare(
                "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id = ?1",
            )?
            .query_row([id.as_bytes()], |row| row.try_into())?;

        Ok(workspace)
    }

    fn insert_command(&self, record: CommandRecord) -> StorageProviderResult<()> {
        self.connection
            .prepare(
                "INSERT INTO commands (
                    id,
                    last_execute_time,
                    name,
                    program,
                    workspace_id
                ) VALUES (?1, ?2, ?3, ?4, ?5)",
            )?
            .execute(params![
                record.id,
                record.last_execute_time,
                record.name,
                record.program,
                record.workspace_id
            ])?;

        Ok(())
    }

    fn insert_workspace(&self, record: WorkspaceRecord) -> StorageProviderResult<()> {
        self.connection
            .prepare(
                "INSERT INTO workspaces (
                    id,
                    last_access_time,
                    location,
                    name
                ) VALUES (?1, ?2, ?3, ?4)",
            )?
            .execute(params![
                record.id,
                record.last_access_time,
                record.location,
                record.name
            ])?;

        Ok(())
    }

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

    fn list_commands_within_workspace(
        &self,
        input: ListCommandsWithinWorkspaceInput,
    ) -> StorageProviderResult<Vec<CommandRecord>> {
        let ListCommandsWithinWorkspaceInput {
            program_contains,
            workspace_id,
            page_number,
            page_size,
        } = input;

        let program_contains = format!("%{}%", program_contains.to_lowercase());

        let mut statement = self.connection.prepare(
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
        )?;

        let records = statement
            .query_map(
                params![
                    program_contains,
                    workspace_id.as_bytes(),
                    page_size,
                    page_number * page_size
                ],
                |row| row.try_into(),
            )?
            .collect::<std::result::Result<Vec<CommandRecord>, _>>()?;

        Ok(records)
    }

    fn list_workspaces(
        &self,
        input: ListWorkspacesInput,
    ) -> StorageProviderResult<Vec<WorkspaceRecord>> {
        let ListWorkspacesInput {
            name_contains,
            page_size,
            page_number,
        } = input;

        let name_contains = format!("%{}%", name_contains.to_lowercase());

        let mut statement = self.connection.prepare(
            "SELECT
                id,
                last_access_time,
                location,
                name
            FROM workspaces
            WHERE LOWER(name) LIKE ?1
            ORDER BY last_access_time DESC, name ASC
            LIMIT ?2 OFFSET ?3",
        )?;

        let records = statement
            .query_map(
                params![name_contains, page_size, page_number * page_size],
                |row| row.try_into(),
            )?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()?;

        Ok(records)
    }

    fn load_rarray(&self) -> rusqlite::Result<()> {
        rusqlite::vtab::array::load_module(self.connection)
    }

    fn migrate(&self) -> rusqlite::Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id BLOB PRIMARY KEY,
                last_access_time INTEGER,
                location TEXT,
                name TEXT NOT NULL
            )",
            (),
        )?;

        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id BLOB PRIMARY KEY,
                last_execute_time INTEGER,
                name TEXT NOT NULL,
                program TEXT NOT NULL,
                workspace_id BLOB NOT NULL
            )",
            (),
        )?;

        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS
            commands_workspace_id_idx
            ON commands(workspace_id)",
            (),
        )?;

        Ok(())
    }

    pub fn new(connection: &'a Connection) -> StorageProviderResult<Self> {
        let provider = Self { connection };

        provider.migrate()?;
        provider.load_rarray()?;

        Ok(provider)
    }

    #[cfg(feature = "extensions")]
    fn track_command_execution_time(
        &self,
        workspace_id: Uuid,
        command_id: Uuid,
    ) -> StorageProviderResult<()> {
        let last_execute_time = Utc::now().timestamp_nanos_opt();

        let mut statement = self.connection.prepare(
            "UPDATE commands
                SET last_execute_time = ?1
                WHERE id = ?2 AND workspace_id = ?3",
        )?;

        statement.execute(params![
            last_execute_time,
            command_id.as_bytes(),
            workspace_id.as_bytes()
        ])?;

        Ok(())
    }

    #[cfg(feature = "extensions")]
    fn track_workspace_access_time(&self, workspace_id: Uuid) -> StorageProviderResult<()> {
        let last_access_time = Utc::now().timestamp_nanos_opt();

        let mut statement = self.connection.prepare(
            "UPDATE workspaces
                SET last_access_time = ?1
                WHERE id = ?2",
        )?;

        statement.execute(params![last_access_time, workspace_id.as_bytes()])?;

        Ok(())
    }

    fn update_command(&self, command: CommandRecord) -> StorageProviderResult<()> {
        let CommandRecord {
            id,
            last_execute_time: _,
            name,
            program,
            workspace_id,
        } = command;

        let mut statement = self.connection.prepare(
            "UPDATE commands
                SET
                    name = ?1,
                    program = ?2
                WHERE id = ?3 AND workspace_id = ?4",
        )?;

        statement.execute(params![name, program, id, workspace_id])?;

        Ok(())
    }

    fn update_workspace(&self, workspace: WorkspaceRecord) -> StorageProviderResult<()> {
        let WorkspaceRecord {
            id,
            last_access_time: _,
            location,
            name,
        } = workspace;

        let mut statement = self.connection.prepare(
            "UPDATE workspaces
                SET
                    location = ?1,
                    name = ?2
                WHERE id = ?3",
        )?;

        statement.execute(params![location, name, id])?;

        Ok(())
    }
}

/// File system location for the terminal app files
pub fn app_path() -> io::Result<PathBuf> {
    let is_release = cfg!(not(debug_assertions));

    let mut app_path = if is_release {
        user_path()?
    } else {
        development_path()?
    };

    app_path.push(".hermione");

    if !app_path.try_exists()? {
        fs::create_dir(&app_path)?;
    }

    Ok(app_path.to_path_buf())
}

fn development_path() -> io::Result<PathBuf> {
    let output = process::Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()?;

    let project_path = str::from_utf8(&output.stdout)
        .map_err(|_err| io::Error::other("Can't read project path"))?;

    Path::new(project_path)
        .parent()
        .map(|path| path.to_path_buf())
        .ok_or(io::Error::other("Missing terminal app development path"))
}

fn user_path() -> io::Result<PathBuf> {
    let dir = dirs::home_dir().ok_or(io::Error::other("Can't get user's home dir"))?;

    Ok(dir)
}

impl From<StorageProviderError> for Error {
    fn from(value: StorageProviderError) -> Self {
        Self::Storage(eyre::Error::from(value.0))
    }
}

impl From<CommandRecord> for Command {
    fn from(value: CommandRecord) -> Self {
        let CommandRecord {
            id,
            last_execute_time,
            name,
            program,
            workspace_id,
        } = value;

        let id = Uuid::from_bytes(id);

        let last_execute_time = last_execute_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        let workspace_id = Uuid::from_bytes(workspace_id);

        Self::load(LoadCommandParameters {
            id,
            last_execute_time,
            name,
            program,
            workspace_id,
        })
    }
}

impl From<WorkspaceRecord> for Workspace {
    fn from(value: WorkspaceRecord) -> Self {
        let WorkspaceRecord {
            id,
            last_access_time,
            location,
            name,
        } = value;

        let id = Uuid::from_bytes(id);

        let last_access_time = last_access_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        Self::load(LoadWorkspaceParameters {
            id,
            last_access_time,
            location,
            name,
        })
    }
}

impl TryFrom<&Command> for CommandRecord {
    type Error = Error;

    fn try_from(value: &Command) -> Result<Self> {
        let id = value.try_id()?.into_bytes();

        let last_execute_time = value
            .last_execute_time()
            .and_then(|date_time| date_time.timestamp_nanos_opt());

        Ok(Self {
            id,
            last_execute_time,
            name: value.name().to_string(),
            program: value.program().to_string(),
            workspace_id: value.workspace_id().into_bytes(),
        })
    }
}

impl TryFrom<&rusqlite::Row<'_>> for CommandRecord {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(CommandRecord {
            id: row.get(0)?,
            last_execute_time: row.get(1)?,
            name: row.get(2)?,
            program: row.get(3)?,
            workspace_id: row.get(4)?,
        })
    }
}

impl TryFrom<&rusqlite::Row<'_>> for WorkspaceRecord {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(WorkspaceRecord {
            id: row.get(0)?,
            last_access_time: row.get(1)?,
            location: row.get(2)?,
            name: row.get(3)?,
        })
    }
}

impl TryFrom<&Workspace> for WorkspaceRecord {
    type Error = hermione_ops::Error;

    fn try_from(value: &Workspace) -> Result<Self> {
        let id = value.try_id()?.into_bytes();

        let last_access_time = value
            .last_access_time()
            .and_then(|date_time| date_time.timestamp_nanos_opt());

        Ok(Self {
            id,
            last_access_time,
            location: value.location().map(ToString::to_string),
            name: value.name().to_string(),
        })
    }
}

impl CreateCommand for StorageProvider<'_> {
    fn create_command(&self, mut command: Command) -> Result<Command> {
        let id = Uuid::new_v4();
        command.set_id(id)?;

        let record = CommandRecord::try_from(&command)?;
        self.insert_command(record)?;

        Ok(command)
    }
}

impl CreateWorkspace for StorageProvider<'_> {
    fn create_workspace(&self, mut workspace: Workspace) -> Result<Workspace> {
        let id = Uuid::new_v4();
        workspace.set_id(id)?;

        let record = WorkspaceRecord::try_from(&workspace)?;

        self.insert_workspace(record)?;

        Ok(workspace)
    }
}

impl DeleteCommandFromWorkspace for StorageProvider<'_> {
    fn delete(&self, scoped_id: CommandWorkspaceScopedId) -> Result<()> {
        let CommandWorkspaceScopedId {
            workspace_id,
            command_id,
        } = scoped_id;

        self.delete_command_from_workspace(workspace_id, command_id)?;

        Ok(())
    }
}

impl DeleteWorkspace for StorageProvider<'_> {
    fn delete(&self, id: Uuid) -> Result<()> {
        self.delete_workspace(id)?;

        Ok(())
    }
}

impl GetCommandFromWorkspace for StorageProvider<'_> {
    fn get_command_from_workspace(&self, scoped_id: CommandWorkspaceScopedId) -> Result<Command> {
        let CommandWorkspaceScopedId {
            workspace_id,
            command_id,
        } = scoped_id;

        let record = self.get_command_from_workspace(workspace_id, command_id)?;

        Ok(record.into())
    }
}

impl GetWorkspace for StorageProvider<'_> {
    fn get_workspace(&self, id: Uuid) -> Result<Workspace> {
        let record = self.get_workspace(id)?;

        Ok(record.into())
    }
}

#[cfg(feature = "backup")]
impl<'a> Import for BackupProvider<'a, Command> {
    type Entity = Command;

    fn import(&self, command: Command) -> impl Future<Output = Result<Command>> {
        let record = match CommandRecord::try_from(&command) {
            Ok(record) => record,
            Err(error) => return future::ready(Err(error)),
        };

        if let Err(err) = self.storage_provider().insert_command(record) {
            return future::ready(Err(err.into()));
        }

        future::ready(Ok(command))
    }
}

#[cfg(feature = "backup")]
impl<'a> Import for BackupProvider<'a, Workspace> {
    type Entity = Workspace;

    fn import(&self, workspace: Workspace) -> impl Future<Output = Result<Workspace>> {
        let record = match WorkspaceRecord::try_from(&workspace) {
            Ok(record) => record,
            Err(error) => return future::ready(Err(error)),
        };

        if let Err(err) = self.storage_provider().insert_workspace(record) {
            return future::ready(Err(err.into()));
        }

        future::ready(Ok(workspace))
    }
}

#[cfg(feature = "backup")]
impl<'a> Iterate for BackupProvider<'a, Workspace> {
    type Entity = Workspace;

    fn iterate(&self) -> impl Future<Output = Result<Option<Vec<Self::Entity>>>> {
        let mut page_number = match self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))
        {
            Ok(page_number) => page_number,
            Err(err) => return future::ready(Err(err.into())),
        };

        let records = match self
            .storage_provider()
            .list_workspaces_by_page(*page_number)
        {
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

#[cfg(feature = "backup")]
impl<'a> Iterate for BackupProvider<'a, Command> {
    type Entity = Command;

    fn iterate(&self) -> impl Future<Output = Result<Option<Vec<Self::Entity>>>> {
        let mut page_number = match self
            .page_number
            .write()
            .map_err(|_err| eyre::Error::msg("Failed to read page number from lock"))
        {
            Ok(page_number) => page_number,
            Err(err) => return future::ready(Err(err.into())),
        };

        let records = match self.storage_provider().list_commands_by_page(*page_number) {
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

impl ListCommandsWithinWorkspace for StorageProvider<'_> {
    fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceParameters,
    ) -> Result<Vec<Command>> {
        let ListCommandsWithinWorkspaceParameters {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let records = self.list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        })?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}

#[cfg(feature = "backup")]
impl<'a> ListByIds for BackupProvider<'a, Command> {
    type Entity = Command;

    fn list_by_ids(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Self::Entity>>> {
        let result = self.storage_provider().list_commands_by_ids(ids);

        let result = result
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(Into::into);

        future::ready(result)
    }
}

#[cfg(feature = "backup")]
impl<'a> ListByIds for BackupProvider<'a, Workspace> {
    type Entity = Workspace;

    fn list_by_ids(&self, ids: Vec<Uuid>) -> impl Future<Output = Result<Vec<Self::Entity>>> {
        let result = self.storage_provider().list_workspaces_by_ids(ids);

        let result = result
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(Into::into);

        future::ready(result)
    }
}

impl ListWorkspaces for StorageProvider<'_> {
    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        let ListWorkspacesParameters {
            name_contains,
            page_size,
            page_number,
        } = parameters;

        let records = self.list_workspaces(ListWorkspacesInput {
            name_contains,
            page_size,
            page_number,
        })?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}

#[cfg(feature = "extensions")]
impl TrackCommandExecutionTime for StorageProvider<'_> {
    fn track_command_execution_time(&self, command: Command) -> Result<Command> {
        let command_id = command.try_id()?;

        self.track_command_execution_time(command_id, command.workspace_id())?;

        GetCommandFromWorkspace::get_command_from_workspace(
            self,
            CommandWorkspaceScopedId {
                command_id,
                workspace_id: command.workspace_id(),
            },
        )
    }
}

#[cfg(feature = "extensions")]
impl TrackWorkspaceAccessTime for StorageProvider<'_> {
    fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace_id = workspace.try_id()?;

        self.track_workspace_access_time(workspace_id)?;

        GetWorkspace::get_workspace(self, workspace_id)
    }
}

impl UpdateCommand for StorageProvider<'_> {
    fn update_command(&self, entity: Command) -> Result<Command> {
        let record = CommandRecord::try_from(&entity)?;

        self.update_command(record)?;

        Ok(entity)
    }
}

impl<'a> Update for BackupProvider<'a, Command> {
    type Entity = Command;

    fn update(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>> {
        let result = UpdateCommand::update_command(&self.storage_provider(), entity);

        future::ready(result)
    }
}

impl<'a> Update for BackupProvider<'a, Workspace> {
    type Entity = Workspace;

    fn update(&self, entity: Self::Entity) -> impl Future<Output = Result<Self::Entity>> {
        let result = UpdateWorkspace::update_workspace(&self.storage_provider(), entity);

        future::ready(result)
    }
}

impl UpdateWorkspace for StorageProvider<'_> {
    fn update_workspace(&self, entity: Workspace) -> Result<Workspace> {
        let record = WorkspaceRecord::try_from(&entity)?;

        self.update_workspace(record)?;

        Ok(entity)
    }
}
