mod test_case;

use crate::support::{InMemoryStorage, NotionBackupCredentialsFixture};
use hermione_nexus::definitions::BackupProviderKind;
use test_case::Background;

#[test]
fn test_delete_backup_credentials_operation_succeeds_for_notion() {
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

    test_case::assert_operation_success(operation_result);
    test_case::assert_storage_does_not_contain_notion_backup_credentials(&background);
}
