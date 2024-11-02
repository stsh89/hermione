use hermione_nexus::{
    definitions::{
        Command, CommandId, CommandParameters, Workspace, WorkspaceId, WorkspaceParameters,
    },
    services::{
        CreateCommand, CreateWorkspace, EditCommandParameters, EditWorkspaceParameters,
        FilterWorkspacesParameters, FindCommand, FindWorkspace, ListWorkspaces,
        NewCommandParameters, NewWorkspaceParameters, StorageProvider, UpdateCommand,
        UpdateWorkspace,
    },
    Error,
};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum InMemoryStorageError {
    #[error("Data integrity: {0}")]
    DataItegrity(String),

    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct InMemoryStorageProvider {
    pub workspaces: RwLock<HashMap<Uuid, Workspace>>,
    pub commands: RwLock<HashMap<Uuid, Command>>,
}

impl InMemoryStorageProvider {
    fn get_command(&self, command_id: &CommandId) -> Result<Option<Command>, InMemoryStorageError> {
        let command = self.commands.read()?.get(command_id).cloned();

        Ok(command)
    }

    fn get_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>, InMemoryStorageError> {
        let workspace = self.workspaces.read()?.get(id).cloned();

        Ok(workspace)
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

    pub fn new() -> Self {
        Self {
            workspaces: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
        }
    }

    fn workspaces(&self) -> Result<Vec<Workspace>, InMemoryStorageError> {
        let workspaces = self.workspaces.read()?;

        Ok(workspaces.values().cloned().collect())
    }
}

impl StorageProvider for InMemoryStorageProvider {}

impl CreateCommand for InMemoryStorageProvider {
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

impl CreateWorkspace for InMemoryStorageProvider {
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

impl FindCommand for InMemoryStorageProvider {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>, Error> {
        let command = self.get_command(id)?;

        Ok(command)
    }
}

impl FindWorkspace for InMemoryStorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>, Error> {
        let workspaces = self.get_workspace(id)?;

        Ok(workspaces)
    }
}

impl ListWorkspaces for InMemoryStorageProvider {
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
            .skip((page_number - 1) as usize * page_size as usize)
            .take(page_size as usize)
            .collect())
    }
}

impl UpdateCommand for InMemoryStorageProvider {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<Command, Error> {
        let EditCommandParameters { id, name, program } = parameters;

        let mut command = self
            .get_command(&id)?
            .ok_or_else(|| Error::NotFound(format!("Command with ID: {}", **id)))?;

        command.set_name(name.to_string());
        command.set_program(program.to_string());

        self.insert_command(&command)?;

        Ok(command.clone())
    }
}

impl UpdateWorkspace for InMemoryStorageProvider {
    fn update_workspace(&self, parameters: EditWorkspaceParameters) -> Result<Workspace, Error> {
        let EditWorkspaceParameters { id, location, name } = parameters;

        let mut workspace = self
            .get_workspace(id)?
            .ok_or_else(|| Error::NotFound(format!("Workspace with ID: {}", **id)))?;

        workspace.set_location(location.map(ToString::to_string));
        workspace.set_name(name.to_string());

        self.insert_workspace(&workspace)?;

        Ok(workspace)
    }
}

impl<T> From<PoisonError<T>> for InMemoryStorageError {
    fn from(err: PoisonError<T>) -> Self {
        Self::LockAccess(err.to_string())
    }
}

impl From<InMemoryStorageError> for Error {
    fn from(err: InMemoryStorageError) -> Self {
        Error::Storage(eyre::Error::new(err))
    }
}
