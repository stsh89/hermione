use crate::Connection;
use chrono::{DateTime, Utc};
use hermione_core::{
    workspaces::{
        CreateWorkspace, CreateWorkspaceOperation, DeleteWorkspace, DeleteWorkspaceOperation,
        FindWorkspace, FindWorkspaceOperation, GetWorkspace, GetWorkspaceOperation,
        ImportWorkspace, ImportWorkspaceOperation, ListWorkspaceOperation, ListWorkspaces,
        ListWorkspacesParameters, LoadWorkspaceParameters, NewWorkspaceParameters,
        TrackWorkspaceAccessTime, TrackWorkspaceAccessTimeOperation, UpdateWorkspace,
        UpdateWorkspaceOperation, Workspace,
    },
    Error,
};
use rusqlite::{params, OptionalExtension, Statement};
use std::str::FromStr;
use uuid::{Bytes, Uuid};

pub struct WorkspaceDto {
    pub id: String,
    pub last_access_time: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub name: String,
}

pub struct WorkspacesClient {
    inner: DatabaseProvider,
}

pub struct ListWorkspacesInput<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

impl WorkspacesClient {
    pub fn create_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = CreateWorkspaceOperation {
            creator: &self.inner,
        }
        .execute(data.new_entity())?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn delete_workspace(&self, id: &str) -> anyhow::Result<()> {
        DeleteWorkspaceOperation {
            deleter: &self.inner,
        }
        .execute(id.parse()?)?;

        Ok(())
    }

    pub fn find(&self, id: &str) -> anyhow::Result<Option<WorkspaceDto>> {
        let workspace = FindWorkspaceOperation {
            finder: &self.inner,
        }
        .execute(id.parse()?)?;

        Ok(workspace.map(WorkspaceDto::from_entity))
    }

    pub fn get_workspace(&self, id: &str) -> anyhow::Result<WorkspaceDto> {
        let workspace = GetWorkspaceOperation {
            getter: &self.inner,
        }
        .execute(id.parse()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn import_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = ImportWorkspaceOperation {
            importer: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }

    pub fn list_workspaces(
        &self,
        parameters: ListWorkspacesInput<'_>,
    ) -> anyhow::Result<Vec<WorkspaceDto>> {
        let ListWorkspacesInput {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = ListWorkspaceOperation {
            lister: &self.inner,
        }
        .execute(ListWorkspacesParameters {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces
            .into_iter()
            .map(WorkspaceDto::from_entity)
            .collect())
    }

    pub fn new(connection: &Connection) -> anyhow::Result<Self> {
        let inner = DatabaseProvider::new(connection)?;

        Ok(Self { inner })
    }

    pub fn track_workspace_access_time(&self, id: &str) -> anyhow::Result<WorkspaceDto> {
        let entity = self.inner.get(Uuid::from_str(id)?)?;

        let entity = TrackWorkspaceAccessTimeOperation {
            tracker: &self.inner,
        }
        .execute(entity)?;

        Ok(WorkspaceDto::from_entity(entity))
    }

    pub fn update_workspace(&self, data: WorkspaceDto) -> anyhow::Result<WorkspaceDto> {
        let workspace = UpdateWorkspaceOperation {
            updater: &self.inner,
        }
        .execute(data.load_entity()?)?;

        Ok(WorkspaceDto::from_entity(workspace))
    }
}

impl WorkspaceRecord {
    pub fn from_entity(workspace: &Workspace) -> eyre::Result<Self> {
        let id = *workspace
            .id()
            .ok_or(eyre::eyre!("Record without id"))?
            .as_bytes();

        let last_access_time = workspace
            .last_access_time()
            .and_then(|date_time| date_time.timestamp_nanos_opt());

        Ok(Self {
            id,
            last_access_time,
            location: workspace.location().map(ToString::to_string),
            name: workspace.name().to_string(),
        })
    }

    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(WorkspaceRecord {
            id: row.get(0)?,
            last_access_time: row.get(1)?,
            location: row.get(2)?,
            name: row.get(3)?,
        })
    }

    pub fn load_entity(self) -> Workspace {
        let WorkspaceRecord {
            id,
            last_access_time,
            location,
            name,
        } = self;

        let id = Uuid::from_bytes(id);

        let last_access_time = last_access_time
            .map(DateTime::from_timestamp_nanos)
            .map(From::from);

        Workspace::load(LoadWorkspaceParameters {
            id,
            last_access_time,
            location,
            name,
        })
    }
}

struct DatabaseProvider {
    connection: rusqlite::Connection,
}

struct WorkspaceRecord {
    id: Bytes,
    last_access_time: Option<i64>,
    location: Option<String>,
    name: String,
}

impl DatabaseProvider {
    pub fn new(connection: &Connection) -> rusqlite::Result<Self> {
        Ok(Self {
            connection: connection.open()?,
        })
    }

    fn insert(&self, record: WorkspaceRecord) -> rusqlite::Result<()> {
        self.connection
            .prepare(
                "INSERT INTO workspaces (
                    id,
                    last_access_time,
                    location,
                    name
                ) VALUES (?1, ?2, ?3, ?4)",
            )?
            .execute(params![
                record.id,
                record.last_access_time,
                record.location,
                record.name
            ])?;

        Ok(())
    }

    fn select_workspace(&self) -> rusqlite::Result<Statement> {
        self.connection.prepare(
            "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE id = ?1",
        )
    }
}

impl CreateWorkspace for DatabaseProvider {
    fn create(&self, mut workspace: Workspace) -> Result<Workspace, Error> {
        let id = Uuid::new_v4();
        workspace.set_id(id)?;

        let record = WorkspaceRecord::from_entity(&workspace)?;

        self.insert(record).map_err(eyre::Error::new)?;

        Ok(workspace)
    }
}

impl DeleteWorkspace for DatabaseProvider {
    fn delete(&self, id: Uuid) -> Result<(), Error> {
        // TODO: apply transaction

        let mut statement = self
            .connection
            .prepare("DELETE FROM workspaces WHERE id = ?1")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes()])
            .map_err(eyre::Error::new)?;

        let mut statement = self
            .connection
            .prepare("DELETE FROM commands WHERE workspace_id = ?1")
            .map_err(eyre::Error::new)?;

        statement
            .execute([id.as_bytes()])
            .map_err(eyre::Error::new)?;

        Ok(())
    }
}

impl FindWorkspace for DatabaseProvider {
    fn find_workspace(&self, id: Uuid) -> Result<Option<Workspace>, Error> {
        let record = self
            .select_workspace()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], WorkspaceRecord::from_row)
            .optional()
            .map_err(eyre::Error::new)?;

        Ok(record.map(WorkspaceRecord::load_entity))
    }
}

impl GetWorkspace for DatabaseProvider {
    fn get(&self, id: Uuid) -> Result<Workspace, Error> {
        let record = self
            .select_workspace()
            .map_err(eyre::Error::new)?
            .query_row([id.as_bytes()], WorkspaceRecord::from_row)
            .map_err(eyre::Error::new)?;

        Ok(WorkspaceRecord::load_entity(record))
    }
}

impl ImportWorkspace for DatabaseProvider {
    fn import(&self, entity: Workspace) -> Result<Workspace, Error> {
        let record = WorkspaceRecord::from_entity(&entity)?;

        self.insert(record).map_err(eyre::Error::new)?;

        Ok(entity)
    }
}

impl ListWorkspaces for DatabaseProvider {
    fn list(&self, parameters: ListWorkspacesParameters) -> Result<Vec<Workspace>, Error> {
        let ListWorkspacesParameters {
            name_contains,
            page_size,
            page_number,
        } = parameters;

        let name_contains = format!("%{}%", name_contains.to_lowercase());

        let mut statement = self
            .connection
            .prepare(
                "SELECT
                    id,
                    last_access_time,
                    location,
                    name
                FROM workspaces
                WHERE LOWER(name) LIKE ?1
                ORDER BY last_access_time DESC, name ASC
                LIMIT ?2 OFFSET ?3",
            )
            .map_err(eyre::Error::new)?;

        let records = statement
            .query_map(
                params![name_contains, page_size, page_number * page_size],
                WorkspaceRecord::from_row,
            )
            .map_err(eyre::Error::new)?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(eyre::Error::new)?;

        let entities = records
            .into_iter()
            .map(WorkspaceRecord::load_entity)
            .collect();

        Ok(entities)
    }
}

impl TrackWorkspaceAccessTime for DatabaseProvider {
    fn track_access_time(&self, workspace: Workspace) -> Result<Workspace, Error> {
        let record = WorkspaceRecord::from_entity(&workspace)?;

        let last_access_time = Utc::now()
            .timestamp_nanos_opt()
            .ok_or(eyre::eyre!("Failed to get timestamp"))?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE workspaces
                SET last_access_time = ?1
                WHERE id = ?2",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![last_access_time, record.id])
            .map_err(eyre::Error::new)?;

        self.get(Uuid::from_bytes(record.id))
    }
}

impl UpdateWorkspace for DatabaseProvider {
    fn update(&self, entity: Workspace) -> Result<Workspace, Error> {
        let record = WorkspaceRecord::from_entity(&entity)?;

        let mut statement = self
            .connection
            .prepare(
                "UPDATE workspaces
                SET
                    location = ?1,
                    name = ?2
                WHERE id = ?3",
            )
            .map_err(eyre::Error::new)?;

        statement
            .execute(params![record.location, record.name, record.id])
            .map_err(eyre::Error::new)?;

        self.get(Uuid::from_bytes(record.id))
    }
}

impl WorkspaceDto {
    fn from_entity(workspace: Workspace) -> Self {
        Self {
            id: workspace.id().map(|id| id.to_string()).unwrap_or_default(),
            last_access_time: workspace.last_access_time().cloned(),
            location: workspace.location().map(ToString::to_string),
            name: workspace.name().to_string(),
        }
    }

    fn load_entity(self) -> anyhow::Result<Workspace> {
        let WorkspaceDto {
            id,
            last_access_time: _,
            location,
            name,
        } = self;

        Ok(Workspace::load(LoadWorkspaceParameters {
            id: Uuid::from_str(&id)?,
            name,
            location,
            last_access_time: self.last_access_time.map(From::from),
        }))
    }

    fn new_entity(self) -> Workspace {
        let WorkspaceDto {
            id: _,
            last_access_time: _,
            location,
            name,
        } = self;

        Workspace::new(NewWorkspaceParameters { name, location })
    }
}
