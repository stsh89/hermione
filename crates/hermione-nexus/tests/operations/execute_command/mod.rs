mod test_case;

use crate::support::{
    CommandFixture, ExpectedCommand, ExpectedWorkspace, InMemoryStorage, MockSystem,
    WorkspaceFixture,
};
use test_case::{
    Background, BackgroundContext, ExpectedOperationResult, ExpectedStorageState,
    ExpectedSystemChanges,
};

#[test]
fn test_execute_command_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
        system: MockSystem::default(),
    };

    test_case::setup(
        &background,
        BackgroundContext {
            workspace: WorkspaceFixture {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                last_access_time: None,
                location: Some("/home/ironman"),
            },
            command: CommandFixture {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Ping",
                program: "ping 1.1.1.1",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
            time_freeze: "2024-11-17 20:20:00",
        },
    );

    let operation_result =
        test_case::execute_operation(&background, "51280bfc-2eea-444a-8df9-a1e7158c2c6b");

    test_case::assert_operation_result(operation_result, ExpectedOperationResult::Success);

    test_case::assert_system_changes(
        &background,
        ExpectedSystemChanges {
            last_executed_program: "ping 1.1.1.1",
            last_visited_location: "/home/ironman",
        },
    );

    test_case::assert_storage_changes(
        &background,
        ExpectedStorageState {
            expected_command: ExpectedCommand {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Ping",
                program: "ping 1.1.1.1",
                last_execute_time: Some("2024-11-17 20:20:01"),
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            },
            expected_workspace: ExpectedWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                last_access_time: Some("2024-11-17 20:20:01"),
                location: Some("/home/ironman"),
            },
        },
    );
}
