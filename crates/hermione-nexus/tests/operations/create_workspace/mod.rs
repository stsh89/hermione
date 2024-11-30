mod test_case;

use crate::support::InMemoryStorage;
use hermione_nexus::operations::CreateWorkspaceParameters;
use test_case::{Background, ExpectedOperationResult, ExpectedStoredWorkspace, ExpectedWorkspace};

#[test]
fn test_create_workspace_operation_succeded() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    let operation_result = test_case::execute_operation(
        &background,
        CreateWorkspaceParameters {
            name: "Ironman".to_string(),
            location: Some("/home/ironman".to_string()),
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_workspace: ExpectedWorkspace {
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
        },
    );

    test_case::assert_storage_contains_workspace(
        &background.storage,
        ExpectedStoredWorkspace {
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: None,
        },
    );
}
