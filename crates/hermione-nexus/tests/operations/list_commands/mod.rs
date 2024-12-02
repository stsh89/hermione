mod test_case;

use crate::support::{CommandFixture, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::operations::ListCommandsParameters;
use std::num::NonZeroU32;
use test_case::{Background, BackgroundContext, ExpectedOperationResult};

#[test]
fn test_list_commands_operation_filteres_by_program() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        BackgroundContext {
            workspace: WorkspaceFixture {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
            commands: vec![
                CommandFixture {
                    id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                    name: "Ping",
                    program: "ping 1.1.1.1",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
                CommandFixture {
                    id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                    name: "Get directory items",
                    program: "GetChild-Item .",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
            ],
        },
    );

    let operation_result = test_case::execute_operation(
        &background,
        ListCommandsParameters {
            program_contains: Some("Item"),
            page_number: None,
            page_size: None,
            workspace_id: None,
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_commands: vec![ExpectedCommand {
                id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                name: "Get directory items",
                program: "GetChild-Item .",
                last_execute_time: None,
                workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            }],
        },
    );
}

#[test]
fn test_list_commands_operation_paginates() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        BackgroundContext {
            workspace: WorkspaceFixture {
                id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                name: "Ironman",
                location: Some("/home/ironman"),
                last_access_time: None,
            },
            commands: vec![
                CommandFixture {
                    id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                    name: "Ping",
                    program: "ping 1.1.1.1",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
                CommandFixture {
                    id: "657acc69-aafe-426d-8496-9859bc40ca62",
                    name: "Get directory items",
                    program: "getchild-item .",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
                CommandFixture {
                    id: "1d0c6b79-2ea9-4291-85bf-84b9412c3a52",
                    name: "Generate new UUID",
                    program: "new-guid",
                    last_execute_time: Some("2024-11-17 11:00:00"),
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
                CommandFixture {
                    id: "12fe0231-2850-4f9b-b11c-844147f50b3d",
                    name: "Lint Rust codebase",
                    program: "becon",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
            ],
        },
    );

    let operation_result = test_case::execute_operation(
        &background,
        ListCommandsParameters {
            program_contains: None,
            page_number: NonZeroU32::new(2),
            page_size: NonZeroU32::new(2),
            workspace_id: None,
        },
    );

    test_case::assert_operation_result(
        operation_result,
        ExpectedOperationResult::Success {
            expected_commands: vec![
                ExpectedCommand {
                    id: "657acc69-aafe-426d-8496-9859bc40ca62",
                    name: "Get directory items",
                    program: "getchild-item .",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
                ExpectedCommand {
                    id: "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                    name: "Ping",
                    program: "ping 1.1.1.1",
                    last_execute_time: None,
                    workspace_id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                },
            ],
        },
    );
}
