mod test_case;

use crate::support::{CommandFixture, InMemoryStorage, MockSystem, WorkspaceFixture};
use test_case::{Background, BackgroundContext};

#[test]
fn test_execute_program_operation_succeeds() {
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

    let operation_result =
        test_case::execute_operation(&background, "51280bfc-2eea-444a-8df9-a1e7158c2c6b");

    test_case::assert_operation_success(operation_result);
    test_case::assert_clipboard_content(&background, "ping 1.1.1.1");
}
