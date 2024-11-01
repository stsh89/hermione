use hermione_nexus::{
    services::storage::{
        CreateWorkspace, CreateWorkspaceParameters, FindWorkspace, UpdateWorkspace,
        UpdateWorkspaceParameters, Workspace, WorkspaceId, WorkspaceParameters,
    },
    Error, StorageProvider,
};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum InMemoryStorageError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct InMemoryStorageProvider {
    pub workspaces: RwLock<HashMap<Uuid, Workspace>>,
}

impl InMemoryStorageProvider {
    fn get_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>, InMemoryStorageError> {
        let workspace = self
            .workspaces
            .read()?
            .get(id)
            .cloned()
            .map(Workspace::from);

        Ok(workspace)
    }

    pub fn insert_workspace(&self, workspace: &Workspace) -> Result<(), InMemoryStorageError> {
        let mut workspaces = self.workspaces.write()?;

        workspaces.insert(**workspace.id(), workspace.clone());

        Ok(())
    }

    pub fn new() -> Self {
        Self {
            workspaces: RwLock::new(HashMap::new()),
        }
    }
}

impl StorageProvider for InMemoryStorageProvider {}

impl CreateWorkspace for InMemoryStorageProvider {
    fn create_workspace(&self, parameters: CreateWorkspaceParameters) -> Result<Workspace, Error> {
        let CreateWorkspaceParameters { name, location } = parameters;

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

impl FindWorkspace for InMemoryStorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>, Error> {
        let workspaces = self.get_workspace(id)?;

        Ok(workspaces)
    }
}

impl UpdateWorkspace for InMemoryStorageProvider {
    fn update_workspace(&self, workspace: UpdateWorkspaceParameters) -> Result<Workspace, Error> {
        let UpdateWorkspaceParameters { id, location, name } = workspace;

        let mut workspaces = self
            .workspaces
            .write()
            .map_err(Into::<InMemoryStorageError>::into)?;

        let workspace = workspaces
            .get_mut(id)
            .ok_or_else(|| Error::NotFound(format!("Workspace with ID: {}", **id)))?;

        workspace.set_location(location.map(ToString::to_string));
        workspace.set_name(name.to_string());

        Ok(workspace.clone())
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
