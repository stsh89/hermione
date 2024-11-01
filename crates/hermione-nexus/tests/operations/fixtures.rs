use chrono::{DateTime, Utc};
use hermione_nexus::{
    definitions::{Workspace, WorkspaceParameters},
    Result,
};
use uuid::Uuid;

#[derive(Default)]
pub struct WorkspaceFixtureParameters {
    pub name: Option<String>,
    pub location: Option<String>,
    pub last_access_time: Option<DateTime<Utc>>,
    pub id: Option<Uuid>,
}

pub fn workspace_fixture(parameters: WorkspaceFixtureParameters) -> Result<Workspace> {
    let WorkspaceFixtureParameters {
        name,
        location,
        last_access_time,
        id,
    } = parameters;

    Workspace::new(WorkspaceParameters {
        id: id.unwrap_or(Uuid::new_v4()),
        last_access_time,
        location,
        name: name.unwrap_or("Test workspace".to_string()),
    })
}
