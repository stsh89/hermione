mod test_case;

use std::num::NonZeroU32;

use crate::support::{ExistingWorkspace, ExpectedWorkspace, InMemoryStorage};
use hermione_nexus::operations::ListWorkspacesParameters;
use test_case::{Background, ExpectedOperationResult};

#[test]
fn test_list_workspace_operation_returns_workspace_filtered_by_name() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        vec![
            ExistingWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
            ExistingWorkspace {
                id: "637d207c-7a18-47eb-b0b4-7f27d4ecbf88",
                name: "Avenger",
                location: None,
                last_access_time: None,
            },
        ],
    );

    let operation_result = test_case::execute_operation(
        &background,
        ListWorkspacesParameters {
            name_contains: Some("man"),
            page_number: None,
            page_size: None,
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_workspaces: vec![ExpectedWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            }],
        },
    );
}

#[test]
fn test_list_workspace_operation_paginates_workspaces() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        vec![
            ExistingWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
            ExistingWorkspace {
                id: "637d207c-7a18-47eb-b0b4-7f27d4ecbf88",
                name: "Avenger",
                location: None,
                last_access_time: None,
            },
            ExistingWorkspace {
                id: "19e0f51f-efaa-4b22-a35d-17c37f350823",
                location: Some("/home/batman"),
                name: "Batman",
                last_access_time: Some("2024-11-17 20:00:00"),
            },
            ExistingWorkspace {
                id: "d9469304-ec44-4c84-8612-7ba3c27b9e29",
                location: Some("/home/vision"),
                name: "Vision",
                last_access_time: None,
            },
        ],
    );

    let operation_result = test_case::execute_operation(
        &background,
        ListWorkspacesParameters {
            name_contains: None,
            page_number: NonZeroU32::new(2),
            page_size: NonZeroU32::new(2),
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_workspaces: vec![
                ExpectedWorkspace {
                    id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                    name: "Ironman",
                    location: Some("/home/ironman"),
                    last_access_time: None,
                },
                ExpectedWorkspace {
                    id: "d9469304-ec44-4c84-8612-7ba3c27b9e29",
                    location: Some("/home/vision"),
                    name: "Vision",
                    last_access_time: None,
                },
            ],
        },
    );
}

#[test]
fn test_list_workspace_operation_sorts_workspaces_by_last_access_time_and_name() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        vec![
            ExistingWorkspace {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
            ExistingWorkspace {
                id: "637d207c-7a18-47eb-b0b4-7f27d4ecbf88",
                name: "Avenger",
                location: None,
                last_access_time: None,
            },
            ExistingWorkspace {
                id: "19e0f51f-efaa-4b22-a35d-17c37f350823",
                location: Some("/home/batman"),
                name: "Batman",
                last_access_time: Some("2024-11-17 20:00:00"),
            },
        ],
    );

    let operation_result = test_case::execute_operation(
        &background,
        ListWorkspacesParameters {
            name_contains: None,
            page_number: NonZeroU32::new(1),
            page_size: NonZeroU32::new(10),
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_workspaces: vec![
                ExpectedWorkspace {
                    id: "19e0f51f-efaa-4b22-a35d-17c37f350823",
                    location: Some("/home/batman"),
                    name: "Batman",
                    last_access_time: Some("2024-11-17 20:00:00"),
                },
                ExpectedWorkspace {
                    id: "637d207c-7a18-47eb-b0b4-7f27d4ecbf88",
                    name: "Avenger",
                    location: None,
                    last_access_time: None,
                },
                ExpectedWorkspace {
                    id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                    name: "Ironman",
                    location: Some("/home/ironman"),
                    last_access_time: None,
                },
            ],
        },
    );
}
