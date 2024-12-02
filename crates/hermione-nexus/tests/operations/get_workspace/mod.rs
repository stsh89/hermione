mod test_case;

use crate::support::{ExpectedWorkspace, InMemoryStorage, WorkspaceFixture};
use test_case::{Background, ExpectedOperationResult};

#[test]
fn test_get_workspace_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        WorkspaceFixture {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: Some("2024-11-17 20:00:00"),
        },
    );

    let operation_result =
        test_case::execute_operation(&background, "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa");

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_workspace: ExpectedWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: Some("2024-11-17 20:00:00"),
            },
        },
    );
}
