mod test_case;

use crate::support::{
    ExpectedNotionBackupCredentials, InMemoryStorage, NotionBackupCredentialsFixture,
};
use hermione_nexus::definitions::BackupProviderKind;
use test_case::Background;

#[test]
fn test_get_backup_credentials_operation_succeeds() {
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

    let operation_result = test_case::execute_operation(&background, BackupProviderKind::Notion);

    test_case::assert_operation_success(
        operation_result,
        ExpectedNotionBackupCredentials {
            api_key: "test_api_key",
            commands_database_id: "test_commands_database_id",
            workspaces_database_id: "test_workspaces_database_id",
        },
    );
}
