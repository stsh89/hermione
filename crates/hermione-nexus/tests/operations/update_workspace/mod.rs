mod test_case;

use crate::support::{self, ExistingWorkspace, ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::operations::UpdateWorkspaceParameters;
use test_case::{Background, OperationResult};

#[test]
fn test_update_workspace_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        ExistingWorkspace {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: Some("2024-11-17 20:00:00"),
        },
    );

    let operation_result = test_case::execute_operation(
        &background,
        UpdateWorkspaceParameters {
            id: support::parse_workspace_id("9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"),
            location: Some("/home/avenger".to_string()),
            name: "Avenger".to_string(),
        },
    );

    test_case::assert_operation_succeess(
        operation_result,
        OperationResult::Success {
            expected_workspace: ExpectedWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Avenger",
                location: Some("/home/avenger"),
                last_access_time: Some("2024-11-17 20:00:00"),
            },
        },
    );
}
