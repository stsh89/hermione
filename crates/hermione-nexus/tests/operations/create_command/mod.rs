mod test_case;

use crate::support::{self, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::operations::CreateCommandParameters;
use test_case::Background;

#[test]
fn test_create_command_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        WorkspaceFixture {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            last_access_time: None,
            location: None,
            name: "Ironman",
        },
    );

    let command = test_case::execute_operation(
        &background,
        CreateCommandParameters {
            name: "Ping".to_string(),
            program: "ping 1.1.1.1".to_string(),
            workspace_id: support::parse_workspace_id("9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"),
        },
    )
    .unwrap();

    let id = command.id().as_uuid().to_string();

    test_case::assert_returned_command(
        command,
        ExpectedCommand {
            id: &id,
            name: "Ping",
            program: "ping 1.1.1.1",
            last_execute_time: None,
            workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        },
    );

    test_case::assert_storage_contains_command(
        &background,
        ExpectedCommand {
            id: &id,
            name: "Ping",
            program: "ping 1.1.1.1",
            last_execute_time: None,
            workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        },
    );
}
