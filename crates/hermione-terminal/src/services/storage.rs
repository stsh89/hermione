use chrono::{DateTime, Utc};
use hermione_drive::sqlite::{
    self, CommandRecord, ListCommandsQuery, ListWorkspacesQuery, WorkspaceRecord,
};
use hermione_nexus::{
    definitions::{
        Command, CommandId, CommandParameters, Workspace, WorkspaceId, WorkspaceParameters,
    },
    services::{
        CreateCommand, CreateWorkspace, DeleteCommand, DeleteWorkspace, DeleteWorkspaceCommands,
        EditCommandParameters, EditWorkspaceParameters, FilterCommandsParameters,
        FilterWorkspacesParameters, FindCommand, FindWorkspace, ListCommands, ListWorkspaces,
        NewCommandParameters, NewWorkspaceParameters, StorageProvider, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime, UpdateCommand, UpdateWorkspace,
    },
    Error, Result,
};
use rusqlite::Connection;
use uuid::Uuid;

pub struct Storage<'a> {
    pub conn: &'a Connection,
}

fn internal_error(err: rusqlite::Error) -> Error {
    Error::Storage(eyre::Error::new(err))
}

fn command_from_record(record: CommandRecord) -> Result<Command> {
    let CommandRecord {
        id,
        last_execute_time,
        name,
        program,
        workspace_id,
    } = record;

    Command::new(CommandParameters {
        id: Uuid::from_bytes(id),
        last_execute_time: last_execute_time.and_then(DateTime::from_timestamp_micros),
        name,
        program,
        workspace_id: Uuid::from_bytes(workspace_id).into(),
    })
}

fn workspace_from_record(record: WorkspaceRecord) -> Result<Workspace> {
    let WorkspaceRecord {
        id,
        last_access_time,
        location,
        name,
    } = record;

    Workspace::new(WorkspaceParameters {
        id: Uuid::from_bytes(id),
        last_access_time: last_access_time.and_then(DateTime::from_timestamp_micros),
        location,
        name,
    })
}

impl StorageProvider for Storage<'_> {}

impl CreateCommand for Storage<'_> {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command> {
        let NewCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        let record = CommandRecord {
            id: Uuid::new_v4().into_bytes(),
            last_execute_time: None,
            name,
            program,
            workspace_id: workspace_id.into_bytes(),
        };

        let command = command_from_record(record.clone())?;

        sqlite::insert_command(self.conn, record).map_err(internal_error)?;

        Ok(command)
    }
}

impl CreateWorkspace for Storage<'_> {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace> {
        let NewWorkspaceParameters { name, location } = parameters;

        let record = WorkspaceRecord {
            id: Uuid::new_v4().into_bytes(),
            last_access_time: None,
            location,
            name,
        };

        let workspace = workspace_from_record(record.clone())?;

        sqlite::insert_workspace(self.conn, record).map_err(internal_error)?;

        Ok(workspace)
    }
}

impl DeleteCommand for Storage<'_> {
    fn delete_command(&self, id: &CommandId) -> Result<()> {
        sqlite::delete_command(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl DeleteWorkspace for Storage<'_> {
    fn delete_workspace(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::delete_workspace(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl DeleteWorkspaceCommands for Storage<'_> {
    fn delete_workspace_commands(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::delete_workspace_commands(self.conn, id.as_bytes()).map_err(internal_error)?;

        Ok(())
    }
}

impl FindCommand for Storage<'_> {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>> {
        let record = sqlite::find_command(self.conn, id.as_bytes()).map_err(internal_error)?;

        record.map(command_from_record).transpose()
    }
}

impl FindWorkspace for Storage<'_> {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>> {
        let record = sqlite::find_workspace(self.conn, id.as_bytes()).map_err(internal_error)?;

        record.map(workspace_from_record).transpose()
    }
}

impl ListCommands for Storage<'_> {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>> {
        let FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        } = parameters;

        let records = sqlite::list_commands(
            self.conn,
            ListCommandsQuery {
                program_contains: program_contains.unwrap_or_default(),
                workspace_id: workspace_id.map(|id| id.into_bytes()),
                offset: page_number,
                limit: page_size,
            },
        )
        .map_err(internal_error)?;

        records
            .into_iter()
            .map(command_from_record)
            .collect::<Result<Vec<_>>>()
    }
}

impl ListWorkspaces for Storage<'_> {
    fn list_workspaces(&self, parameters: FilterWorkspacesParameters) -> Result<Vec<Workspace>> {
        let FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let records = sqlite::list_workspaces(
            self.conn,
            ListWorkspacesQuery {
                name_contains: name_contains.unwrap_or_default(),
                limit: page_size,
                offset: page_number,
            },
        )
        .map_err(internal_error)?;

        records
            .into_iter()
            .map(workspace_from_record)
            .collect::<Result<Vec<_>>>()
    }
}

impl TrackCommandExecuteTime for Storage<'_> {
    fn track_command_execute_time(&self, id: &CommandId) -> Result<()> {
        sqlite::refresh_command_execute_time(
            self.conn,
            id.as_bytes(),
            Utc::now().timestamp_micros(),
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl TrackWorkspaceAccessTime for Storage<'_> {
    fn track_workspace_access_time(&self, id: &WorkspaceId) -> Result<()> {
        sqlite::refresh_workspace_access_time(
            self.conn,
            id.as_bytes(),
            Utc::now().timestamp_micros(),
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpdateCommand for Storage<'_> {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<()> {
        let EditCommandParameters { id, name, program } = parameters;

        let record = sqlite::find_command(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .ok_or(Error::NotFound(format!("Command {}", Uuid::nil().braced())))?;

        sqlite::update_command(
            self.conn,
            CommandRecord {
                name: name.to_string(),
                program: program.to_string(),
                ..record
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}

impl UpdateWorkspace for Storage<'_> {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<()> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let record = sqlite::find_workspace(self.conn, id.as_bytes())
            .map_err(internal_error)?
            .ok_or(Error::NotFound(format!(
                "Workspace {}",
                Uuid::nil().braced()
            )))?;

        sqlite::update_workspace(
            self.conn,
            WorkspaceRecord {
                location: location.map(ToString::to_string),
                name: name.to_string(),
                ..record
            },
        )
        .map_err(internal_error)?;

        Ok(())
    }
}
