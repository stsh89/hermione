mod test_case;

use crate::support::{self, InMemoryStorage, MockSystem, WorkspaceFixture};
use hermione_nexus::operations::ExecuteProgramParameters;
use test_case::{Background, ExpectedOperationResult};

#[test]
fn test_execute_program_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
        system: MockSystem::default(),
    };

    test_case::setup(
        &background,
        WorkspaceFixture {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            name: "Ironman",
            last_access_time: None,
            location: None,
        },
    );

    let operation_result = test_case::execute_operation(
        &background,
        ExecuteProgramParameters {
            program: "ping 1.1.1.1",
            workspace_id: Some(support::parse_workspace_id(
                "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            )),
        },
    );

    test_case::assert_operation_result(operation_result, ExpectedOperationResult::Success);
    test_case::assert_executed_system_program(&background, "ping 1.1.1.1");
}
