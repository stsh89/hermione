use crate::Connection;
use chrono::{DateTime, Utc};
use hermione_core::{
    commands::{
        Command, CommandWorkspaceScopeId, CreateCommand, CreateCommandOperation,
        DeleteCommandFromWorkspace, DeleteCommandFromWorkspaceOperation, FindCommandInWorkspace,
        FindCommandInWorkspaceOperation, GetCommandFromWorkspaceOperation, GetCommandInWorkspace,
        ImportCommand, ImportCommandOperation, ListCommands, ListCommandsOperation,
        ListCommandsParameters, ListCommandsWithinWorkspace, ListCommandsWithinWorkspaceOperation,
        ListCommandsWithinWorkspaceParameters, LoadCommandParameters, NewCommandParameters,
        TrackCommandExecutionTime, TrackCommandExecutionTimeOperation, UpdateCommand,
        UpdateCommandOperation,
    },
    Error,
};
use rusqlite::{params, OptionalExtension, Statement};
use uuid::{Bytes, Uuid};

pub struct CommandDto {
    pub id: String,
    pub last_execute_time: Option<DateTime<Utc>>,
    pub name: String,
    pub program: String,
    pub workspace_id: String,
}

pub struct ListCommandsInput {
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListCommandsWithinWorkspaceInput<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

pub struct CommandsClient {
    inner: DatabaseProvider,
}

impl CommandsClient {
    pub fn create_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = CreateCommandOperation {
            creator: &self.inner,
        }
        .execute(data.new_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn delete_command_from_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<()> {
        let id = CommandWorkspaceScopeId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        DeleteCommandFromWorkspaceOperation {
            deleter: &self.inner,
        }
        .execute(id)?;

        Ok(())
    }

    pub fn find_command_in_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<Option<CommandDto>> {
        let id = CommandWorkspaceScopeId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = FindCommandInWorkspaceOperation {
            finder: &self.inner,
        }
        .execute(id)?;

        Ok(command.map(CommandDto::from_entity))
    }

    pub fn get_command_from_workspace(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<CommandDto> {
        let id = CommandWorkspaceScopeId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let command = GetCommandFromWorkspaceOperation {
            getter: &self.inner,
        }
        .execute(id)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn import_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = ImportCommandOperation {
            importer: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }

    pub fn list_commands(&self, parameters: ListCommandsInput) -> anyhow::Result<Vec<CommandDto>> {
        let ListCommandsInput {
            page_number,
            page_size,
        } = parameters;

        let workspaces = ListCommandsOperation {
            lister: &self.inner,
        }
        .execute(ListCommandsParameters {
            page_number,
            page_size,
        })?;

        Ok(workspaces
            .into_iter()
            .map(CommandDto::from_entity)
            .collect())
    }

    pub fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceInput,
    ) -> anyhow::Result<Vec<CommandDto>> {
        let ListCommandsWithinWorkspaceInput {
            page_number,
            page_size,
            program_contains,
            workspace_id,
        } = parameters;

        let workspaces = ListCommandsWithinWorkspaceOperation {
            lister: &self.inner,
        }
        .execute(ListCommandsWithinWorkspaceParameters {
            page_number,
            page_size,
            program_contains,
            workspace_id: workspace_id.parse()?,
        })?;

        Ok(workspaces
            .into_iter()
            .map(CommandDto::from_entity)
            .collect())
    }

    pub fn new(connection: &Connection) -> anyhow::Result<Self> {
        let inner = DatabaseProvider::new(connection)?;

        Ok(Self { inner })
    }

    pub fn track_command_execution_time(
        &self,
        workspace_id: &str,
        id: &str,
    ) -> anyhow::Result<CommandDto> {
        let id = CommandWorkspaceScopeId {
            workspace_id: workspace_id.parse()?,
            command_id: id.parse()?,
        };

        let entity = self.inner.get(id)?;

        let entity = TrackCommandExecutionTimeOperation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(CommandDto::from_entity(entity))
    }

    pub fn update_command(&self, data: CommandDto) -> anyhow::Result<CommandDto> {
        let command = UpdateCommandOperation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(CommandDto::from_entity(command))
    }
}

impl CommandDto {
    fn from_entity(command: Command) -> Self {
        Self {
            id: command.id().map(|id| id.to_string()).unwrap_or_default(),
            name: command.name().to_string(),
            last_execute_time: command.last_execute_time().cloned(),
            program: command.program().to_string(),
            workspace_id: command.workspace_id().to_string(),
        }
    }

    fn load_entity(self) -> anyhow::Result<Command> {
        let CommandDto {
            id,
            name,
            last_execute_time,
            program,
            workspace_id,
        } = self;

        Ok(Command::load(LoadCommandParameters {
            id: id.parse()?,
            name,
            last_execute_time: last_execute_time.map(From::from),
            program,
            workspace_id: workspace_id.parse()?,
        }))
    }

    fn new_entity(self) -> anyhow::Result<Command> {
        let CommandDto {
            id: _,
            name,
            last_execute_time: _,
            program,
            workspace_id,
        } = self;

        Ok(Command::new(NewCommandParameters {
            name,
            program,
            workspace_id: workspace_id.parse()?,
        }))
    }
}

struct DatabaseProvider {
    connection: rusqlite::Connection,
}

struct CommandRecord {
    id: Bytes,
    last_execute_time: Option<i64>,
    name: String,
    program: String,
    workspace_id: Bytes,
}

impl DatabaseProvider {
    fn new(connection: &Connection) -> rusqlite::Result<Self> {
        Ok(Self {
            connection: connection.open()?,
        })
    }

    fn insert(&self, record: CommandRecord) -> rusqlite::Result<()> {
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

    fn select_command(&self) -> rusqlite::Result<Statement> {
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
}

impl CreateCommand for DatabaseProvider {
    fn create(&self, mut command: Command) -> Result<Command, Error> {
        let id = Uuid::new_v4();
        command.set_id(id)?;

        let record = CommandRecord::from_entity(&command)?;
        self.insert(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl DeleteCommandFromWorkspace for DatabaseProvider {
    fn delete(&self, id: CommandWorkspaceScopeId) -> Result<(), Error> {
        let CommandWorkspaceScopeId {
            workspace_id,
            command_id: id,
        } = id;

        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE id = ?1 AND workspace_id = ?2")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes(), workspace_id.as_bytes()])
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl FindCommandInWorkspace for DatabaseProvider {
    fn find(&self, id: CommandWorkspaceScopeId) -> Result<Option<Command>, Error> {
        let CommandWorkspaceScopeId {
            workspace_id,
            command_id: id,
        } = id;

        let record = self
            .select_command()
            .map_err(eyre::Error::new)?
            .query_row(
                [id.as_bytes(), workspace_id.as_bytes()],
                CommandRecord::from_row,
            )
            .optional()
            .map_err(eyre::Error::new)?;

        Ok(record.map(CommandRecord::load_entity))
    }
}

impl GetCommandInWorkspace for DatabaseProvider {
    fn get(&self, id: CommandWorkspaceScopeId) -> Result<Command, Error> {
        let CommandWorkspaceScopeId {
            workspace_id,
            command_id: id,
        } = id;

        let record = self
            .select_command()
            .map_err(eyre::Error::new)?
            .query_row(
                [id.as_bytes(), workspace_id.as_bytes()],
                CommandRecord::from_row,
            )
            .map_err(eyre::Error::new)?;

        Ok(CommandRecord::load_entity(record))
    }
}

impl ImportCommand for DatabaseProvider {
    fn import(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        self.insert(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl ListCommands for DatabaseProvider {
    fn list(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>, Error> {
        let ListCommandsParameters {
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
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![page_size, page_number * page_size],
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

impl ListCommandsWithinWorkspace for DatabaseProvider {
    fn list(
        &self,
        parameters: ListCommandsWithinWorkspaceParameters,
    ) -> Result<Vec<Command>, Error> {
        let ListCommandsWithinWorkspaceParameters {
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

impl TrackCommandExecutionTime for DatabaseProvider {
    fn track_command_execution_time(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

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
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_execute_time, record.id, record.workspace_id])
            .map_err(eyre::Error::new)?;

        self.get(CommandWorkspaceScopeId {
            command_id: Uuid::from_bytes(record.id),
            workspace_id: Uuid::from_bytes(record.workspace_id),
        })
    }
}

impl UpdateCommand for DatabaseProvider {
    fn update(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        let mut statement = self
            .connection
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

        self.get(CommandWorkspaceScopeId {
            command_id: Uuid::from_bytes(record.id),
            workspace_id: Uuid::from_bytes(record.workspace_id),
        })
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
