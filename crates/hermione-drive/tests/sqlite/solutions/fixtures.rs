use hermione_drive::sqlite::workspaces::WorkspaceRecord;
use uuid::{Bytes, Uuid};

#[derive(Default)]
pub struct WorkspaceRecordFixtureParameters {
    pub id: Option<Bytes>,
    pub last_access_time: Option<i64>,
    pub location: Option<String>,
    pub name: Option<String>,
}

pub fn workspace_record_fixture(parameters: WorkspaceRecordFixtureParameters) -> WorkspaceRecord {
    let WorkspaceRecordFixtureParameters {
        id,
        last_access_time,
        location,
        name,
    } = parameters;

    WorkspaceRecord {
        id: id.unwrap_or_else(|| Uuid::new_v4().into_bytes()),
        last_access_time,
        location,
        name: name.unwrap_or_else(|| "Test workspace".into()),
    }
}
