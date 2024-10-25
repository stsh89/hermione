use chrono::DateTime;
use hermione_ops::{
    commands::{
        Command, CommandWorkspaceScopedId, CreateCommand, DeleteCommandFromWorkspace, FindCommand,
        GetCommandFromWorkspace, ListCommandsWithinWorkspace,
        ListCommandsWithinWorkspaceParameters, LoadCommandParameters, UpdateCommand,
    },
    workspaces::{
        CreateWorkspace, DeleteWorkspace, FindWorkspace, GetWorkspace, ListAllWorkspacesInBatches,
        ListWorkspaces, ListWorkspacesParameters, LoadWorkspaceParameters, UpdateWorkspace,
        Workspace,
    },
    Result,
};
use rusqlite::{params, Connection, OptionalExtension, Statement};
use std::path::Path;
use uuid::{Bytes, Uuid};

const DEFAULT_PAGE_SIZE: u32 = 100;

pub(crate) struct CommandRecord {
    pub(crate) id: Bytes,
    pub(crate) last_execute_time: Option<i64>,
    pub(crate) name: String,
    pub(crate) program: String,
    pub(crate) workspace_id: Bytes,
}

pub(crate) struct WorkspaceRecord {
    pub(crate) id: Bytes,
    pub(crate) last_access_time: Option<i64>,
    pub(crate) location: Option<String>,
    pub(crate) name: String,
}

pub struct DatabaseProvider {
    connection: Connection,
}

impl DatabaseProvider {
    pub(crate) fn connection(&self) -> &Connection {
        &self.connection
    }

    pub(crate) fn insert_command(&self, record: CommandRecord) -> rusqlite::Result<()> {
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

    pub(crate) fn insert_workspace(&self, record: WorkspaceRecord) -> rusqlite::Result<()> {
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

    fn load_rarray(&self) -> rusqlite::Result<()> {
        rusqlite::vtab::array::load_module(self.connection())
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

    pub fn new(file_path: &Path) -> rusqlite::Result<Self> {
        let connection = Connection::open(file_path)?;
        let provider = Self { connection };

        provider.migrate()?;
        provider.load_rarray()?;

        Ok(provider)
    }

    fn select_command_within_workspace_statement(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE id = ?1 AND workspace_id = ?2",
        )
    }

    fn select_command_statement(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                id,
                last_execute_time,
                name,
                program,
                workspace_id
            FROM commands
            WHERE id = ?1",
        )
    }

    fn select_workspace_statement(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id = ?1",
        )
    }
}

impl CommandRecord {
    pub fn from_entity(command: &Command) -> eyre::Result<Self> {
        let id = *command
            .id()
            .ok_or(eyre::eyre!("Command entity without id"))?
            .as_bytes();

        let last_execute_time = command
            .last_execute_time()
            .and_then(|date_time| date_time.timestamp_nanos_opt());

        Ok(Self {
            id,
            last_execute_time,
            name: command.name().to_string(),
            program: command.program().to_string(),
            workspace_id: *command.workspace_id().as_bytes(),
        })
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(CommandRecord {
            id: row.get(0)?,
            last_execute_time: row.get(1)?,
            name: row.get(2)?,
            program: row.get(3)?,
            workspace_id: row.get(4)?,
        })
    }

    pub fn load_entity(self) -> Command {
        let CommandRecord {
            id: _,
            last_execute_time: _,
            name,
            program,
            workspace_id: _,
        } = self;

        let id = Uuid::from_bytes(self.id);

        let last_execute_time = self
            .last_execute_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        let workspace_id = Uuid::from_bytes(self.workspace_id);

        Command::load(LoadCommandParameters {
            id,
            last_execute_time,
            name,
            program,
            workspace_id,
        })
    }
}

impl TryFrom<&Workspace> for WorkspaceRecord {
    type Error = eyre::Error;

    fn try_from(value: &Workspace) -> eyre::Result<Self, Self::Error> {
        let id = value
            .id()
            .ok_or(eyre::eyre!("Record without id"))?
            .into_bytes();

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

impl TryFrom<Workspace> for WorkspaceRecord {
    type Error = eyre::Error;

    fn try_from(value: Workspace) -> eyre::Result<Self, Self::Error> {
        TryFrom::try_from(&value)
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

        Workspace::load(LoadWorkspaceParameters {
            id,
            last_access_time,
            location,
            name,
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

impl CreateCommand for DatabaseProvider {
    fn create_command(&self, mut command: Command) -> Result<Command> {
        let id = Uuid::new_v4();
        command.set_id(id)?;

        let record = CommandRecord::from_entity(&command)?;
        self.insert_command(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl CreateWorkspace for DatabaseProvider {
    fn create_workspace(&self, mut workspace: Workspace) -> Result<Workspace> {
        let id = Uuid::new_v4();
        workspace.set_id(id)?;

        let record = WorkspaceRecord::try_from(&workspace)?;

        self.insert_workspace(record).map_err(eyre::Error::new)?;

        Ok(workspace)
    }
}

impl DeleteCommandFromWorkspace for DatabaseProvider {
    fn delete(&self, id: CommandWorkspaceScopedId) -> Result<()> {
        let CommandWorkspaceScopedId {
            workspace_id,
            command_id: id,
        } = id;

        let mut statement = self
            .connection()
            .prepare("DELETE FROM commands WHERE id = ?1 AND workspace_id = ?2")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes(), workspace_id.as_bytes()])
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl DeleteWorkspace for DatabaseProvider {
    fn delete(&self, id: Uuid) -> Result<()> {
        // TODO: apply transaction

        let mut statement = self
            .connection()
            .prepare("DELETE FROM workspaces WHERE id = ?1")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes()])
            .map_err(eyre::Error::new)?;

        let mut statement = self
            .connection()
            .prepare("DELETE FROM commands WHERE workspace_id = ?1")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes()])
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl FindCommand for DatabaseProvider {
    fn find_command(&self, id: Uuid) -> Result<Option<Command>> {
        let record = self
            .select_command_statement()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], CommandRecord::from_row)
            .optional()
            .map_err(eyre::Error::new)?;

        Ok(record.map(CommandRecord::load_entity))
    }
}

impl FindWorkspace for DatabaseProvider {
    fn find_workspace(&self, id: Uuid) -> Result<Option<Workspace>> {
        let record: Option<WorkspaceRecord> = self
            .select_workspace_statement()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], |row| row.try_into())
            .optional()
            .map_err(eyre::Error::new)?;

        Ok(record.map(Into::into))
    }
}

impl GetCommandFromWorkspace for DatabaseProvider {
    fn get_command_from_workspace(&self, id: CommandWorkspaceScopedId) -> Result<Command> {
        let CommandWorkspaceScopedId {
            workspace_id,
            command_id: id,
        } = id;

        let record = self
            .select_command_within_workspace_statement()
            .map_err(eyre::Error::new)?
            .query_row(
                [id.as_bytes(), workspace_id.as_bytes()],
                CommandRecord::from_row,
            )
            .map_err(eyre::Error::new)?;

        Ok(CommandRecord::load_entity(record))
    }
}

impl GetWorkspace for DatabaseProvider {
    fn get_workspace(&self, id: Uuid) -> Result<Workspace> {
        let record: WorkspaceRecord = self
            .select_workspace_statement()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], |row| row.try_into())
            .map_err(eyre::Error::new)?;

        Ok(record.into())
    }
}

impl ListCommandsWithinWorkspace for DatabaseProvider {
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

        let program_contains = format!("%{}%", program_contains.to_lowercase());

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
                WHERE LOWER(program) LIKE ?1 AND workspace_id = ?2
                ORDER BY last_execute_time DESC, program ASC
                LIMIT ?3 OFFSET ?4",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![
                    program_contains,
                    workspace_id.as_bytes(),
                    page_size,
                    page_number * page_size
                ],
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
}

impl ListWorkspaces for DatabaseProvider {
    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        let ListWorkspacesParameters {
            name_contains,
            page_size,
            page_number,
        } = parameters;

        let name_contains = format!("%{}%", name_contains.to_lowercase());

        let mut statement = self
            .connection()
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
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![name_contains, page_size, page_number * page_size],
                |row| row.try_into(),
            )
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<WorkspaceRecord>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records.into_iter().map(Into::into).collect();

        Ok(entities)
    }
}

impl ListAllWorkspacesInBatches for DatabaseProvider {
    async fn list_all_workspaces_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Workspace>) -> hermione_ops::Result<()>,
    ) -> Result<()> {
        let mut page_number = 0;

        loop {
            let commands = self.list_workspaces(ListWorkspacesParameters {
                page_number,
                page_size: DEFAULT_PAGE_SIZE,
                name_contains: "",
            })?;

            if commands.is_empty() {
                break;
            }

            batch_fn(commands)?;

            page_number += 1;
        }

        Ok(())
    }
}

impl UpdateCommand for DatabaseProvider {
    fn update_command(&self, command: Command) -> Result<Command> {
        let record = CommandRecord::from_entity(&command)?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE commands
                SET
                    name = ?1,
                    program = ?2
                WHERE id = ?3 AND workspace_id = ?4",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![
                record.name,
                record.program,
                record.id,
                record.workspace_id
            ])
            .map_err(eyre::Error::new)?;

        self.get_command_from_workspace(CommandWorkspaceScopedId {
            command_id: Uuid::from_bytes(record.id),
            workspace_id: Uuid::from_bytes(record.workspace_id),
        })
    }
}

impl UpdateWorkspace for DatabaseProvider {
    fn update_workspace(&self, entity: Workspace) -> Result<Workspace> {
        let record = WorkspaceRecord::try_from(&entity)?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE workspaces
                SET
                    location = ?1,
                    name = ?2
                WHERE id = ?3",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![record.location, record.name, record.id])
            .map_err(eyre::Error::new)?;

        self.get_workspace(Uuid::from_bytes(record.id))
    }
}
