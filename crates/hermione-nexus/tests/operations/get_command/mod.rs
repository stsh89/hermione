mod test_case;

use crate::support::{ExistingCommand, ExistingWorkspace, ExpectedCommand, InMemoryStorage};
use test_case::{Background, ExistingStorageData, ExpectedOperationResult};

#[test]
fn test_get_command_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        ExistingStorageData {
            workspace: ExistingWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                last_access_time: None,
                location: None,
            },
            command: ExistingCommand {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Ping",
                program: "ping 1.1.1.1",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
        },
    );

    let operation_result =
        test_case::execute_operation(&background, "51280bfc-2eea-444a-8df9-a1e7158c2c6b");

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_command: ExpectedCommand {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Ping",
                program: "ping 1.1.1.1",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
        },
    );
}
