mod test_case;

use crate::support::{self, CommandFixture, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::operations::UpdateCommandParameters;
use test_case::{Background, ExistingStorageData, OperationResult};

#[test]
fn test_update_command_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        ExistingStorageData {
            workspace: WorkspaceFixture {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                last_access_time: None,
                location: None,
            },
            command: CommandFixture {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Ping",
                program: "ping 1.1.1.1",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
        },
    );

    let operation_result = test_case::execute_operation(
        &background,
        UpdateCommandParameters {
            id: support::parse_command_id("51280bfc-2eea-444a-8df9-a1e7158c2c6b"),
            name: "List directory items".to_string(),
            program: "ls -la".to_string(),
        },
    );

    test_case::assert_operation_succeess(
        operation_result,
        OperationResult::Success {
            expected_command: ExpectedCommand {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "List directory items",
                program: "ls -la",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
        },
    );
}
