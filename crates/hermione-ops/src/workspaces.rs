use crate::{Error, Result};
use chrono::{DateTime, Utc};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

pub trait CreateWorkspace {
    fn create_workspace(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait DeleteWorkspace {
    fn delete(&self, id: WorkspaceId) -> Result<()>;
}

pub trait GetWorkspace {
    fn get_workspace(&self, id: &WorkspaceId) -> Result<Workspace>;
}

pub trait ListWorkspaces {
    fn list_workspaces(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait UpdateWorkspace {
    fn update_workspace(&self, workspace: Workspace) -> Result<Workspace>;
}

pub struct CreateWorkspaceOperation<'a, S> {
    pub creator: &'a S,
}

pub struct DeleteWorkspaceOperation<'a, D> {
    pub deleter: &'a D,
}

pub struct GetWorkspaceOperation<'a, R> {
    pub getter: &'a R,
}

pub struct ListWorkspaceOperation<'a, L> {
    pub lister: &'a L,
}

pub struct UpdateWorkspaceOperation<'a, GWP, UWP> {
    pub get_workspace_provider: &'a GWP,
    pub update_workspace_provider: &'a UWP,
}

pub struct Workspace {
    id: WorkspaceId,
    last_load_time: Option<DateTime<Utc>>,
    location: Option<WorkspaceLocation>,
    name: WorkspaceName,
}

struct WorkspaceLocation {
    value: String,
}

struct WorkspaceName {
    value: String,
}

pub struct NewWorkspaceParameters {
    pub name: String,
    pub location: Option<String>,
}

pub struct LoadWorkspaceParameters {
    pub id: Uuid,
    pub last_access_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub name: String,
}

pub struct ListWorkspacesParameters<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

pub struct UpdateWorkspaceParameters<'a> {
    pub id: &'a WorkspaceId,
    pub location: String,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct WorkspaceId(Uuid);

impl<'a, S> CreateWorkspaceOperation<'a, S>
where
    S: CreateWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        if !workspace.id().is_nil() {
            return Err(Error::FailedPrecondition(
                "Workspace id is already set".to_string(),
            ));
        }

        let workspace = self.creator.create_workspace(workspace)?;

        if workspace.id().is_nil() {
            return Err(Error::Internal(
                "Failed to create workspace: id is not set".to_string(),
            ));
        };

        Ok(workspace)
    }
}

impl<'a, D> DeleteWorkspaceOperation<'a, D>
where
    D: DeleteWorkspace,
{
    pub fn execute(&self, workspace_id: WorkspaceId) -> Result<()> {
        self.deleter.delete(workspace_id)
    }
}

impl<'a, R> GetWorkspaceOperation<'a, R>
where
    R: GetWorkspace,
{
    pub fn execute(&self, id: &WorkspaceId) -> Result<Workspace> {
        self.getter.get_workspace(id)
    }
}

impl<'a, L> ListWorkspaceOperation<'a, L>
where
    L: ListWorkspaces,
{
    pub fn execute(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        self.lister.list_workspaces(parameters)
    }
}

impl<'a, GWP, UWP> UpdateWorkspaceOperation<'a, GWP, UWP>
where
    GWP: GetWorkspace,
    UWP: UpdateWorkspace,
{
    pub fn execute(&self, parameters: UpdateWorkspaceParameters) -> Result<Workspace> {
        let UpdateWorkspaceParameters { id, location, name } = parameters;

        let mut workspace = GetWorkspaceOperation {
            getter: self.get_workspace_provider,
        }
        .execute(id)?;

        workspace.rename(name);
        workspace.change_location(location);

        self.update_workspace_provider.update_workspace(workspace)
    }
}

impl Workspace {
    pub fn change_location(&mut self, location: String) {
        if location.is_empty() {
            self.unset_location();
        } else {
            self.set_location(location);
        }
    }

    pub fn last_access_time(&self) -> Option<&DateTime<Utc>> {
        self.last_load_time.as_ref()
    }

    pub fn load(parameters: LoadWorkspaceParameters) -> Self {
        let LoadWorkspaceParameters {
            id,
            last_access_time: last_load_time,
            location,
            name,
        } = parameters;

        Self {
            id: WorkspaceId(id),
            last_load_time,
            location: location.map(|l| WorkspaceLocation { value: l }),
            name: WorkspaceName { value: name },
        }
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_ref().map(|l| l.value.as_str())
    }

    pub fn id(&self) -> &WorkspaceId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name.value
    }

    pub fn new(parameters: NewWorkspaceParameters) -> Self {
        let NewWorkspaceParameters { name, location } = parameters;

        Self {
            id: WorkspaceId(Uuid::nil()),
            last_load_time: None,
            location: location.map(|l| WorkspaceLocation { value: l }),
            name: WorkspaceName { value: name },
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = WorkspaceName { value: name };
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<()> {
        if !self.id.is_nil() {
            return Err(Error::Internal("Workspace id is already set".to_string()));
        }

        self.id = WorkspaceId(id);

        Ok(())
    }

    fn set_location(&mut self, location: String) {
        self.location = Some(WorkspaceLocation { value: location });
    }

    fn unset_location(&mut self) {
        self.location = None;
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        *self.id == *other.id
            && self.name.value == other.name.value
            && self.location.as_ref().map(|l| &l.value) == other.location.as_ref().map(|l| &l.value)
    }
}

impl Deref for WorkspaceId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for WorkspaceId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let value: Uuid = s.parse().map_err(eyre::Error::new)?;

        Ok(Self(value))
    }
}

impl From<Uuid> for WorkspaceId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
