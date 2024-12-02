use crate::support::{InMemoryStorage, MockSystem, WorkspaceFixture};
use test_case::Background;

mod test_case;

#[test]
fn test_visit_workspace_operation_succeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
        system: MockSystem::default(),
    };

    test_case::setup(
        &background,
        WorkspaceFixture {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            name: "Ironman",
            location: Some("/home/ironman"),
            last_access_time: None,
        },
    );

    let operation_result =
        test_case::execute_operation(&background, "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa");

    test_case::assert_operation_success(operation_result);
    test_case::assert_system_location_changed(&background, "/home/ironman");
}
