use crate::{Error, Result};
use chrono::{DateTime, Utc};
use eyre::eyre;
use std::fmt::{self, Debug, Display, Formatter};
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
    pub fn id(&self) -> WorkspaceId {
        self.id
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
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn into_bytes(self) -> [u8; 16] {
        self.0.into_bytes()
    }

    pub fn new(id: Uuid) -> Result<Self> {
        if id.is_nil() {
            return Err(Error::invalid_argument(eyre!(
                "Invalid workspace ID. Workspace ID cannot be nil"
            )));
        }

        Ok(Self(id))
    }

    pub fn parse_str(value: &str) -> Result<Self> {
        let id = Uuid::parse_str(value).map_err(|err| {
            let err = eyre::Error::new(err).wrap_err("Invalid workspace ID representation");
            Error::invalid_argument(err)
        })?;

        Self::new(id)
    }
}

impl Debug for WorkspaceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "workspace:{}", self.0)
    }
}

impl Display for WorkspaceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
