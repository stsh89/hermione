mod test_case;

use crate::support::InMemoryStorage;
use test_case::{Background, ExistingWorkspace};

#[test]
fn test_delete_workspace_operation_succeded() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        ExistingWorkspace {
            id: "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            name: "Ironman",
        },
    );

    let operation_result =
        test_case::execute_operation(&background, "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa");

    test_case::assert_operation_success(operation_result);
    test_case::assert_storage_does_not_contain_workspace(
        &background,
        "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
    );
}
