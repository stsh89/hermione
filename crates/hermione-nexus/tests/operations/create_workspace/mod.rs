mod test_case;

use crate::support::{ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::operations::CreateWorkspaceParameters;
use test_case::Background;

#[test]
fn test_create_workspace_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    let workspace = test_case::execute_operation(
        &background,
        CreateWorkspaceParameters {
            name: "Ironman".to_string(),
            location: Some("/home/ironman".to_string()),
        },
    )
    .unwrap();

    let id = workspace.id().as_uuid().to_string();

    test_case::assert_returned_workspace(
        workspace,
        ExpectedWorkspace {
            id: &id,
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: None,
        },
    );

    test_case::assert_storage_contains_workspace(
        &background,
        ExpectedWorkspace {
            id: &id,
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: None,
        },
    );
}
