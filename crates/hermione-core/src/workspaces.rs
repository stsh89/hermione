use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Error, Result};

pub trait CreateWorkspace {
    fn create(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait DeleteWorkspace {
    fn delete(&self, workspace_id: Uuid) -> Result<()>;
}

pub trait FindWorkspace {
    fn find_workspace(&self, workspace_id: Uuid) -> Result<Option<Workspace>>;
}

pub trait GetWorkspace {
    fn get_workspace(&self, workspace_id: Uuid) -> Result<Workspace>;
}

pub trait ImportWorkspace {
    fn import(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait ListWorkspaces {
    fn list(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>>;
}

pub trait TrackWorkspaceAccessTime {
    fn track_access_time(&self, workspace: Workspace) -> Result<Workspace>;
}

pub trait UpdateWorkspace {
    fn update(&self, workspace: Workspace) -> Result<Workspace>;
}

pub struct CreateWorkspaceOperation<'a, S> {
    pub creator: &'a S,
}

pub struct DeleteWorkspaceOperation<'a, D> {
    pub deleter: &'a D,
}

pub struct FindWorkspaceOperation<'a, R> {
    pub finder: &'a R,
}

pub struct GetWorkspaceOperation<'a, R> {
    pub getter: &'a R,
}

pub struct ImportWorkspaceOperation<'a, S> {
    pub importer: &'a S,
}

pub struct ListWorkspaceOperation<'a, L> {
    pub lister: &'a L,
}

pub struct TrackWorkspaceAccessTimeOperation<'a, T> {
    pub tracker: &'a T,
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

        let workspace = self.creator.create(workspace)?;

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

impl<'a, R> FindWorkspaceOperation<'a, R>
where
    R: FindWorkspace,
{
    pub fn execute(&self, workspace_id: Uuid) -> Result<Option<Workspace>> {
        self.finder.find_workspace(workspace_id)
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

impl<'a, S> ImportWorkspaceOperation<'a, S>
where
    S: ImportWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        self.importer.import(workspace)
    }
}

impl<'a, L> ListWorkspaceOperation<'a, L>
where
    L: ListWorkspaces,
{
    pub fn execute(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>> {
        self.lister.list(parameters)
    }
}

impl<'a, T> TrackWorkspaceAccessTimeOperation<'a, T>
where
    T: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        let time = workspace.last_access_time().cloned();

        let workspace = self.tracker.track_access_time(workspace)?;
        let error_message = "Failed to track workspace access time".to_string();

        if let Some(new_time) = workspace.last_access_time() {
            if let Some(time) = time {
                if time >= *new_time {
                    return Err(Error::Internal(error_message));
                }
            }
        } else {
            return Err(Error::Internal(error_message));
        }

        Ok(workspace)
    }
}

impl<'a, U> UpdateWorkspaceOperation<'a, U>
where
    U: UpdateWorkspace,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        self.updater.update(workspace)
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
}
