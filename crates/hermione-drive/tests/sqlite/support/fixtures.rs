use hermione_drive::sqlite::{CommandRecord, WorkspaceRecord};
use uuid::{Bytes, Uuid};

#[derive(Default)]
pub struct CommandRecordFixtureParameters {
    pub id: Option<Bytes>,
    pub last_execute_time: Option<i64>,
    pub name: Option<String>,
    pub program: Option<String>,
}

#[derive(Default)]
pub struct WorkspaceRecordFixtureParameters {
    pub id: Option<Bytes>,
    pub last_access_time: Option<i64>,
    pub location: Option<String>,
    pub name: Option<String>,
}

pub fn command_record_fixture(
    workspace: &WorkspaceRecord,
    parameters: CommandRecordFixtureParameters,
) -> CommandRecord {
    let CommandRecordFixtureParameters {
        id,
        last_execute_time,
        name,
        program,
    } = parameters;

    CommandRecord {
        id: id.unwrap_or_else(|| Uuid::new_v4().into_bytes()),
        last_execute_time,
        name: name.unwrap_or_else(|| "Test command".into()),
        program: program.unwrap_or_else(|| "echo \"Hello, world!\"".into()),
        workspace_id: workspace.id,
    }
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
