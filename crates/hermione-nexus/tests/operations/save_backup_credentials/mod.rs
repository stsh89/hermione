mod test_case;

use crate::support::{ExpectedNotionBackupCredentials, InMemoryStorage};
use hermione_nexus::definitions::{BackupCredentials, NotionBackupCredentialsParameters};
use test_case::Background;

#[test]
fn test_save_backup_credentials_operation_succeeds_for_notion() {
    let background = Background {
        storage: InMemoryStorage::empty(),
    };

    let operation_result = test_case::execute_operation(
        &background,
        BackupCredentials::notion(NotionBackupCredentialsParameters {
            api_key: "test_api_key".to_string(),
            commands_database_id: "test_commands_database_id".to_string(),
            workspaces_database_id: "test_workspaces_database_id".to_string(),
        }),
    );

    test_case::assert_operation_success(operation_result);

    test_case::assert_storage_contains_notion_backup_credentials(
        &background,
        ExpectedNotionBackupCredentials {
            api_key: "test_api_key",
            commands_database_id: "test_commands_database_id",
            workspaces_database_id: "test_workspaces_database_id",
        },
    );
}
