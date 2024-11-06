use chrono::Utc;
use hermione_nexus::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId, CommandParameters, Workspace,
        WorkspaceId, WorkspaceParameters,
    },
    services::{
        CreateCommand, CreateWorkspace, DeleteBackupCredentials, DeleteCommand, DeleteWorkspace,
        DeleteWorkspaceCommands, EditCommandParameters, EditWorkspaceParameters,
        FilterCommandsParameters, FilterWorkspacesParameters, FindBackupCredentials, FindCommand,
        FindWorkspace, ListBackupCredentials, ListCommands, ListWorkspaces, NewCommandParameters,
        NewWorkspaceParameters, SaveBackupCredentials, StorageProvider, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime, UpdateCommand, UpdateWorkspace, UpsertCommands, UpsertWorkspaces,
    },
    Error,
};
use std::{
    collections::HashMap,
    num::NonZeroU32,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum InMemoryStorageError {
    #[error("Data integrity error: {0}")]
    DataItegrity(String),

    #[error("Invalid page number: {0}")]
    InvalidPageNumber(u32),

    #[error("Invalid page size: {0}")]
    InvalidPageSize(u32),

    #[error("Memory access error: {0}")]
    MemoryAccess(String),

    #[error("{entry_name} entry with ID {entry_id} is missing")]
    MissingEntry {
        entry_name: &'static str,
        entry_id: Uuid,
    },
}

#[derive(Default)]
pub struct InMemoryStorage {
    backup_credentials: RwLock<HashMap<String, BackupCredentials>>,
    commands: RwLock<HashMap<Uuid, Command>>,
    workspaces: RwLock<HashMap<Uuid, Workspace>>,
}

impl InMemoryStorage {
    pub fn backup_credentials(&self) -> Result<Vec<BackupCredentials>, InMemoryStorageError> {
        let credentials = self.backup_credentials.read()?;

        Ok(credentials.values().cloned().collect())
    }

    pub fn commands(&self) -> Result<Vec<Command>, InMemoryStorageError> {
        let commands = self.commands.read()?;

        Ok(commands.values().cloned().collect())
    }

    pub fn count_backup_credentials(&self) -> Result<usize, InMemoryStorageError> {
        let count = self.backup_credentials.read()?.len();

        Ok(count)
    }

    pub fn count_commands(&self) -> Result<usize, InMemoryStorageError> {
        let commands = self.commands.read()?;

        Ok(commands.len())
    }

    pub fn count_workspaces(&self) -> Result<usize, InMemoryStorageError> {
        let workspaces = self.workspaces.read()?;

        Ok(workspaces.len())
    }

    fn get_backup_credentials(
        &self,
        kind: &str,
    ) -> Result<Option<BackupCredentials>, InMemoryStorageError> {
        let credentials = self.backup_credentials.read()?.get(kind).cloned();

        Ok(credentials)
    }

    pub fn get_command(&self, id: &Uuid) -> Result<Option<Command>, InMemoryStorageError> {
        let command = self.commands.read()?.get(id).cloned();

        Ok(command)
    }

    pub fn get_workspace(&self, id: &Uuid) -> Result<Option<Workspace>, InMemoryStorageError> {
        let workspace = self.workspaces.read()?.get(id).cloned();

        Ok(workspace)
    }

    pub fn insert_backup_credentials(
        &self,
        credentials: BackupCredentials,
    ) -> Result<(), InMemoryStorageError> {
        let mut collection = self.backup_credentials.write()?;

        collection.insert(
            credentials.provider_kind().as_str().to_string(),
            credentials,
        );

        Ok(())
    }

    pub fn insert_command(&self, command: &Command) -> Result<(), InMemoryStorageError> {
        let mut commands = self.commands.write()?;

        if self.get_workspace(command.workspace_id())?.is_none() {
            return Err(InMemoryStorageError::DataItegrity(
                "Attempt to add command to non-existent workspace".to_string(),
            ));
        }

        commands.insert(**command.id(), command.clone());

        Ok(())
    }

    pub fn insert_workspace(&self, workspace: &Workspace) -> Result<(), InMemoryStorageError> {
        let mut workspaces = self.workspaces.write()?;

        workspaces.insert(**workspace.id(), workspace.clone());

        Ok(())
    }

    fn remove_backup_credentials(&self, kind: &str) -> Result<(), InMemoryStorageError> {
        let mut credentials = self.backup_credentials.write()?;

        credentials.remove(kind);

        Ok(())
    }

    fn remove_command(&self, id: &Uuid) -> Result<(), InMemoryStorageError> {
        let mut commands = self.commands.write()?;

        commands.remove(id);

        Ok(())
    }

    fn remove_workspace_commands(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Result<(), InMemoryStorageError> {
        let mut commands = self.commands.write()?;

        commands.retain(|_id, command| command.workspace_id() != workspace_id);

        Ok(())
    }

    fn remove_workspace(&self, id: &Uuid) -> Result<(), InMemoryStorageError> {
        let mut workspace = self
            .workspaces
            .write()
            .map_err(Into::<InMemoryStorageError>::into)?;

        workspace.remove(id);

        Ok(())
    }

    fn set_command_execute_time(&self, id: &Uuid) -> Result<(), InMemoryStorageError> {
        let command = self.get_command(id)?;

        let Some(mut command) = command else {
            return Ok(());
        };

        command.set_execute_time(Utc::now());

        self.insert_command(&command)?;

        Ok(())
    }

    fn set_workspace_access_time(&self, id: &Uuid) -> Result<(), InMemoryStorageError> {
        let workspace = self.get_workspace(id)?;

        let Some(mut workspace) = workspace else {
            return Ok(());
        };

        workspace.set_access_time(Utc::now());

        self.insert_workspace(&workspace)?;

        Ok(())
    }

    pub fn workspaces(&self) -> Result<Vec<Workspace>, InMemoryStorageError> {
        let workspaces = self.workspaces.read()?;

        Ok(workspaces.values().cloned().collect())
    }
}

pub fn page_size(size: u32) -> Result<NonZeroU32, InMemoryStorageError> {
    NonZeroU32::new(size).ok_or(InMemoryStorageError::InvalidPageSize(size))
}

pub fn page_number(size: u32) -> Result<NonZeroU32, InMemoryStorageError> {
    NonZeroU32::new(size).ok_or(InMemoryStorageError::InvalidPageNumber(size))
}

impl<T> From<PoisonError<T>> for InMemoryStorageError {
    fn from(err: PoisonError<T>) -> Self {
        Self::MemoryAccess(err.to_string())
    }
}

impl From<InMemoryStorageError> for Error {
    fn from(err: InMemoryStorageError) -> Self {
        Self::Storage(eyre::Error::new(err))
    }
}

impl StorageProvider for InMemoryStorage {}

impl CreateCommand for InMemoryStorage {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command, Error> {
        let NewCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        let command = Command::new(CommandParameters {
            id: Uuid::new_v4(),
            last_execute_time: None,
            name,
            program,
            workspace_id,
        })?;

        self.insert_command(&command)?;

        Ok(command)
    }
}

impl CreateWorkspace for InMemoryStorage {
    fn create_workspace(&self, parameters: NewWorkspaceParameters) -> Result<Workspace, Error> {
        let NewWorkspaceParameters { name, location } = parameters;

        let workspace = Workspace::new(WorkspaceParameters {
            id: Uuid::new_v4(),
            last_access_time: None,
            location,
            name,
        })?;

        self.insert_workspace(&workspace)?;

        Ok(workspace)
    }
}

impl DeleteBackupCredentials for InMemoryStorage {
    fn delete_backup_credentials(&self, kind: &BackupProviderKind) -> hermione_nexus::Result<()> {
        self.remove_backup_credentials(kind.as_str())?;

        Ok(())
    }
}

impl DeleteCommand for InMemoryStorage {
    fn delete_command(&self, id: &CommandId) -> hermione_nexus::Result<()> {
        self.remove_command(id)?;

        Ok(())
    }
}

impl DeleteWorkspaceCommands for InMemoryStorage {
    fn delete_workspace_commands(&self, id: &WorkspaceId) -> hermione_nexus::Result<()> {
        self.remove_workspace_commands(id)?;

        Ok(())
    }
}

impl DeleteWorkspace for InMemoryStorage {
    fn delete_workspace(&self, id: &WorkspaceId) -> hermione_nexus::Result<()> {
        self.remove_workspace(id)?;

        Ok(())
    }
}

impl FindBackupCredentials for InMemoryStorage {
    fn find_backup_credentials(
        &self,
        kind: &BackupProviderKind,
    ) -> Result<Option<BackupCredentials>, Error> {
        let credentials = self.get_backup_credentials(kind.as_str())?;

        Ok(credentials)
    }
}

impl FindCommand for InMemoryStorage {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>, Error> {
        let command = self.get_command(id)?;

        Ok(command)
    }
}

impl FindWorkspace for InMemoryStorage {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>, Error> {
        let workspaces = self.get_workspace(id)?;

        Ok(workspaces)
    }
}

impl ListBackupCredentials for InMemoryStorage {
    fn list_backup_credentials(&self) -> Result<Vec<BackupCredentials>, Error> {
        let credentials = self.backup_credentials()?;

        Ok(credentials)
    }
}

impl ListCommands for InMemoryStorage {
    fn list_commands(&self, parameters: FilterCommandsParameters) -> Result<Vec<Command>, Error> {
        let FilterCommandsParameters {
            program_contains,
            page_number,
            page_size,
            workspace_id,
        } = parameters;

        let mut commands = self
            .commands()?
            .into_iter()
            .filter(|command| {
                let contains_program = if let Some(program_contains) = program_contains {
                    command.program().contains(program_contains)
                } else {
                    true
                };

                let from_workspace = if let Some(workspace_id) = workspace_id {
                    command.workspace_id() == workspace_id
                } else {
                    true
                };

                contains_program && from_workspace
            })
            .collect::<Vec<Command>>();

        commands.sort_by(|a, b| a.program().cmp(b.program()));
        commands.sort_by(|a, b| a.last_execute_time().cmp(&b.last_execute_time()).reverse());

        Ok(commands
            .into_iter()
            .skip(page_number as usize * page_size as usize)
            .take(page_size as usize)
            .collect())
    }
}

impl ListWorkspaces for InMemoryStorage {
    fn list_workspaces(
        &self,
        parameters: FilterWorkspacesParameters,
    ) -> Result<Vec<Workspace>, Error> {
        let FilterWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let mut workspaces = self
            .workspaces()?
            .into_iter()
            .filter(|workspace| {
                if let Some(name_contains) = name_contains {
                    workspace.name().contains(name_contains)
                } else {
                    true
                }
            })
            .collect::<Vec<Workspace>>();

        workspaces.sort_by(|a, b| a.name().cmp(b.name()));
        workspaces.sort_by(|a, b| a.last_access_time().cmp(&b.last_access_time()).reverse());

        Ok(workspaces
            .into_iter()
            .skip(page_number as usize * page_size as usize)
            .take(page_size as usize)
            .collect())
    }
}

impl SaveBackupCredentials for InMemoryStorage {
    fn save_backup_credentials(&self, credentials: &BackupCredentials) -> Result<(), Error> {
        self.insert_backup_credentials(credentials.clone())?;

        Ok(())
    }
}

impl TrackCommandExecuteTime for InMemoryStorage {
    fn track_command_execute_time(&self, id: &CommandId) -> Result<(), Error> {
        self.set_command_execute_time(id)?;

        Ok(())
    }
}

impl TrackWorkspaceAccessTime for InMemoryStorage {
    fn track_workspace_access_time(&self, id: &WorkspaceId) -> Result<(), Error> {
        self.set_workspace_access_time(id)?;

        Ok(())
    }
}

impl UpdateCommand for InMemoryStorage {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<(), Error> {
        let EditCommandParameters { id, name, program } = parameters;

        let mut command = self
            .get_command(id)?
            .ok_or_else(|| Error::NotFound(format!("Command with ID: {}", **id)))?;

        command.set_name(name.to_string());
        command.set_program(program.to_string());

        self.insert_command(&command)?;

        Ok(())
    }
}

impl UpsertCommands for InMemoryStorage {
    fn upsert_commands(&self, commands: Vec<Command>) -> Result<(), Error> {
        for command in commands {
            self.insert_command(&command)?;
        }

        Ok(())
    }
}

impl UpdateWorkspace for InMemoryStorage {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<(), Error> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let mut workspace = self
            .get_workspace(id)?
            .ok_or_else(|| Error::NotFound(format!("Workspace with ID: {}", **id)))?;

        workspace.set_location(location.map(ToString::to_string));
        workspace.set_name(name.to_string());

        self.insert_workspace(&workspace)?;

        Ok(())
    }
}

impl UpsertWorkspaces for InMemoryStorage {
    fn upsert_workspaces(&self, workspaces: Vec<Workspace>) -> Result<(), Error> {
        for workspace in workspaces {
            self.insert_workspace(&workspace)?;
        }

        Ok(())
    }
}
