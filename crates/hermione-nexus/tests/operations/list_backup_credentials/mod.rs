mod test_case;

use crate::support::{
    ExpectedBackupCredentials, ExpectedNotionBackupCredentials, InMemoryStorage,
    NotionBackupCredentialsFixture,
};
use test_case::Background;

#[test]
fn test_list_backup_credentials_operation_succeeds() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    test_case::setup(
        &background,
        NotionBackupCredentialsFixture {
            api_key: "test_api_key",
            commands_database_id: "test_commands_database_id",
            workspaces_database_id: "test_workspaces_database_id",
        },
    );

    let operation_result = test_case::execute_operation(&background);

    test_case::assert_operation_success(
        operation_result,
        vec![ExpectedBackupCredentials::Notion(
            ExpectedNotionBackupCredentials {
                api_key: "test_api_key",
                commands_database_id: "test_commands_database_id",
                workspaces_database_id: "test_workspaces_database_id",
            },
        )],
    );
}
