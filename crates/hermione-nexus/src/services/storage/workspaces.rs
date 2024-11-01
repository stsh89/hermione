use crate::{Error, Result, StorageProvider};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait CreateWorkspace: StorageProvider {
    fn create_workspace(&self, parameters: CreateWorkspaceParameters) -> Result<Workspace>;
}

pub trait FindWorkspace: StorageProvider {
    fn find_workspace(&self, id: &WorkspaceId) -> Result<Option<Workspace>>;
}

pub trait UpdateWorkspace: StorageProvider {
    fn update_workspace(&self, workspace: UpdateWorkspaceParameters) -> Result<Workspace>;
}

pub struct CreateWorkspaceParameters {
    pub name: String,
    pub location: Option<String>,
}

pub struct UpdateWorkspaceParameters<'a> {
    pub id: &'a WorkspaceId,
    pub name: &'a str,
    pub location: Option<&'a str>,
}

#[derive(Clone)]
pub struct WorkspaceId(Uuid);

#[derive(Clone)]
pub struct Workspace {
    id: WorkspaceId,
    last_access_time: Option<DateTime<Utc>>,
    location: Option<WorkspaceLocation>,
    name: WorkspaceName,
}

pub struct WorkspaceParameters {
    pub id: Uuid,
    pub last_access_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub name: String,
}

#[derive(Clone)]
struct WorkspaceLocation {
    value: String,
}

#[derive(Clone)]
struct WorkspaceName {
    value: String,
}

impl Workspace {
    pub fn id(&self) -> &WorkspaceId {
        &self.id
    }

    pub fn last_access_time(&self) -> Option<&DateTime<Utc>> {
        self.last_access_time.as_ref()
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_ref().map(|l| l.value.as_str())
    }

    pub fn name(&self) -> &str {
        &self.name.value
    }

    pub fn new(parameters: WorkspaceParameters) -> Result<Self> {
        let WorkspaceParameters {
            id,
            last_access_time,
            location,
            name,
        } = parameters;

        let mut workspace = Self {
            id: WorkspaceId::new(id)?,
            last_access_time,
            location: None,
            name: WorkspaceName { value: name },
        };

        workspace.set_location(location);

        Ok(workspace)
    }

    pub fn set_location(&mut self, location: Option<String>) {
        let location = location.unwrap_or_default();

        if location.is_empty() {
            self.location = None;
        } else {
            self.location = Some(WorkspaceLocation { value: location });
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = WorkspaceName { value: name };
    }
}

impl WorkspaceId {
    fn new(value: Uuid) -> Result<Self> {
        if value.is_nil() {
            return Err(Error::Internal("Invalid workspace ID".to_string()));
        }

        Ok(Self(value))
    }
}

impl std::ops::Deref for WorkspaceId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
