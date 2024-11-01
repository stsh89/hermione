use hermione_nexus::{
    services::storage::{
        CreateWorkspace, CreateWorkspaceParameters, Workspace, WorkspaceParameters,
    },
    Error, StorageProvider,
};
use std::{
    collections::HashMap,
    sync::{PoisonError, RwLock},
};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
enum InMemoryStorageError {
    #[error("Lock access: {0}")]
    LockAccess(String),
}

pub struct InMemoryStorageProvider {
    pub workspaces: RwLock<HashMap<Uuid, Workspace>>,
}

impl InMemoryStorageProvider {
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

        let mut workspaces = self
            .workspaces
            .write()
            .map_err(Into::<InMemoryStorageError>::into)?;

        let id = Uuid::new_v4();

        let workspace = Workspace::new(WorkspaceParameters {
            id,
            last_access_time: None,
            location,
            name,
        })?;

        workspaces.insert(id, workspace.clone());

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
