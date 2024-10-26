use crate::{Error, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait CreateWorkspace {
    fn create_workspace(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait DeleteWorkspace {
    fn delete(&self, workspace_id: Uuid) -> Result<()>;
}

pub trait GetWorkspace {
    fn get_workspace(&self, workspace_id: Uuid) -> Result<Workspace>;
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

pub struct UpdateWorkspaceOperation<'a, U> {
    pub updater: &'a U,
}

pub struct Workspace {
    id: Option<Uuid>,
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

impl<'a, S> CreateWorkspaceOperation<'a, S>
where
    S: CreateWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        if workspace.id().is_some() {
            return Err(Error::FailedPrecondition(
                "Workspace id is already set".to_string(),
            ));
        }

        let workspace = self.creator.create_workspace(workspace)?;

        if workspace.id().is_none() {
            return Err(Error::Internal(
                "Failed to create workspace: workspace id is not set".to_string(),
            ));
        };

        Ok(workspace)
    }
}

impl<'a, D> DeleteWorkspaceOperation<'a, D>
where
    D: DeleteWorkspace,
{
    pub fn execute(&self, workspace_id: Uuid) -> Result<()> {
        self.deleter.delete(workspace_id)
    }
}

impl<'a, R> GetWorkspaceOperation<'a, R>
where
    R: GetWorkspace,
{
    pub fn execute(&self, workspace_id: Uuid) -> Result<Workspace> {
        self.getter.get_workspace(workspace_id)
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

impl<'a, U> UpdateWorkspaceOperation<'a, U>
where
    U: UpdateWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        self.updater.update_workspace(workspace)
    }
}

impl Workspace {
    pub fn change_location(&mut self, location: String) {
        self.location = Some(WorkspaceLocation { value: location });
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
            id: Some(id),
            last_load_time,
            location: location.map(|l| WorkspaceLocation { value: l }),
            name: WorkspaceName { value: name },
        }
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_ref().map(|l| l.value.as_str())
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name.value
    }

    pub fn new(parameters: NewWorkspaceParameters) -> Self {
        let NewWorkspaceParameters { name, location } = parameters;

        Self {
            id: None,
            last_load_time: None,
            location: location.map(|l| WorkspaceLocation { value: l }),
            name: WorkspaceName { value: name },
        }
    }

    pub fn rename(&mut self, name: String) {
        self.name = WorkspaceName { value: name };
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::Internal("Workspace id is already set".to_string()));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn try_id(&self) -> Result<Uuid> {
        self.id
            .ok_or(Error::DataLoss("Missing workspace ID".into()))
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name.value == other.name.value
            && self.location.as_ref().map(|l| &l.value) == other.location.as_ref().map(|l| &l.value)
    }
}
