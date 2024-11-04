use crate::{Error, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

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

#[derive(Clone, Debug, PartialEq)]
pub struct WorkspaceId(Uuid);

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

    pub fn set_access_time(&mut self, time: DateTime<Utc>) {
        self.last_access_time = Some(time);
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
            return Err(Error::InvalidArgument(
                "Workspace ID cannot be nil".to_string(),
            ));
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

impl From<Uuid> for WorkspaceId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
