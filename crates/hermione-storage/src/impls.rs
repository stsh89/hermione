use crate::{CommandRecord, StorageProvider, WorkspaceRecord};
use chrono::{DateTime, Utc};
use hermione_core::{
    commands::{
        Command, CommandWorkspaceScopedId, CreateCommand, DeleteCommandFromWorkspace,
        FindCommandInWorkspace, GetCommandFromWorkspace, ImportCommand, ListCommands,
        ListCommandsParameters, ListCommandsWithinWorkspace, ListCommandsWithinWorkspaceParameters,
        LoadCommandParameters, TrackCommandExecutionTime, UpdateCommand,
    },
    workspaces::{
        CreateWorkspace, DeleteWorkspace, FindWorkspace, GetWorkspace, ImportWorkspace,
        ListWorkspaces, ListWorkspacesParameters, LoadWorkspaceParameters,
        TrackWorkspaceAccessTime, UpdateWorkspace, Workspace,
    },
    Error,
};
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

impl CreateCommand for StorageProvider {
    fn create(&self, mut command: Command) -> Result<Command, Error> {
        let id = Uuid::new_v4();
        command.set_id(id)?;

        let record = CommandRecord::from_entity(&command)?;
        self.insert_command(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl CreateWorkspace for StorageProvider {
    fn create(&self, mut workspace: Workspace) -> Result<Workspace, Error> {
        let id = Uuid::new_v4();
        workspace.set_id(id)?;

        let record = WorkspaceRecord::try_from(&workspace)?;

        self.insert_workspace(record).map_err(eyre::Error::new)?;

        Ok(workspace)
    }
}

impl DeleteCommandFromWorkspace for StorageProvider {
    fn delete(&self, id: CommandWorkspaceScopedId) -> Result<(), Error> {
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

impl DeleteWorkspace for StorageProvider {
    fn delete(&self, id: Uuid) -> Result<(), Error> {
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

impl FindCommandInWorkspace for StorageProvider {
    fn find(&self, id: CommandWorkspaceScopedId) -> Result<Option<Command>, Error> {
        let CommandWorkspaceScopedId {
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

impl FindWorkspace for StorageProvider {
    fn find_workspace(&self, id: Uuid) -> Result<Option<Workspace>, Error> {
        let record: Option<WorkspaceRecord> = self
            .select_workspace_statement()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], |row| row.try_into())
            .optional()
            .map_err(eyre::Error::new)?;

        Ok(record.map(Into::into))
    }
}

impl GetCommandFromWorkspace for StorageProvider {
    fn get_command_from_workspace(&self, id: CommandWorkspaceScopedId) -> Result<Command, Error> {
        let CommandWorkspaceScopedId {
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

impl GetWorkspace for StorageProvider {
    fn get_workspace(&self, id: Uuid) -> Result<Workspace, Error> {
        let record: WorkspaceRecord = self
            .select_workspace_statement()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], |row| row.try_into())
            .map_err(eyre::Error::new)?;

        Ok(record.into())
    }
}

impl ImportCommand for StorageProvider {
    fn import(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        self.insert_command(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl ImportWorkspace for StorageProvider {
    fn import(&self, entity: Workspace) -> Result<Workspace, Error> {
        let record = WorkspaceRecord::try_from(&entity)?;

        self.insert_workspace(record).map_err(eyre::Error::new)?;

        Ok(entity)
    }
}

impl ListCommands for StorageProvider {
    fn list(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>, Error> {
        let ListCommandsParameters {
            page_number,
            page_size,
        } = parameters;

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

impl ListCommandsWithinWorkspace for StorageProvider {
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

impl TrackCommandExecutionTime for StorageProvider {
    fn track_command_execution_time(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        let last_execute_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE commands
                SET last_execute_time = ?1
                WHERE id = ?2 AND workspace_id = ?3",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_execute_time, record.id, record.workspace_id])
            .map_err(eyre::Error::new)?;

        self.get_command_from_workspace(CommandWorkspaceScopedId {
            command_id: Uuid::from_bytes(record.id),
            workspace_id: Uuid::from_bytes(record.workspace_id),
        })
    }
}

impl UpdateCommand for StorageProvider {
    fn update(&self, command: Command) -> Result<Command, Error> {
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

impl ListWorkspaces for StorageProvider {
    fn list(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>, Error> {
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

impl TrackWorkspaceAccessTime for StorageProvider {
    fn track_access_time(&self, workspace: Workspace) -> Result<Workspace, Error> {
        let record: WorkspaceRecord = workspace.try_into()?;

        let last_access_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection()
            .prepare(
                "UPDATE workspaces
                SET last_access_time = ?1
                WHERE id = ?2",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_access_time, record.id])
            .map_err(eyre::Error::new)?;

        self.get_workspace(Uuid::from_bytes(record.id))
    }
}

impl UpdateWorkspace for StorageProvider {
    fn update(&self, entity: Workspace) -> Result<Workspace, Error> {
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

    fn try_from(row: &rusqlite::Row) -> Result<Self, Self::Error> {
        Ok(WorkspaceRecord {
            id: row.get(0)?,
            last_access_time: row.get(1)?,
            location: row.get(2)?,
            name: row.get(3)?,
        })
    }
}
