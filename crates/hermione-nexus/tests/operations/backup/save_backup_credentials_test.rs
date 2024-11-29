use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage},
};
use hermione_nexus::{
    definitions::{BackupCredentials, BackupProviderKind, NotionBackupCredentialsParameters},
    operations::{SaveBackupCredentialsOperation, SaveBackupCredentialsOperationParameters},
};
use std::rc::Rc;

#[derive(Default)]
struct SaveBackupCredentialsTestCase {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
    operation: Operation<()>,
}

impl TestCase for SaveBackupCredentialsTestCase {
    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let storage_table = parameters.get_table("storage");
        let credentials_table = storage_table.get_table("backup_credentials");
        let backup_provider_name = credentials_table.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        match backup_provider_kind {
            BackupProviderKind::Notion => {
                let backup_credentials = support::get_notion_backup_credentials(&self.storage);

                support::assert_notion_backup_credentials(&backup_credentials, credentials_table);
            }
        };
    }

    fn execute_operation(&mut self, parameters: Table) {
        let backup_provider_name = parameters.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        let backup_credentials = match backup_provider_kind {
            BackupProviderKind::Notion => {
                let api_key = parameters.get_text("api_key");
                let commands_database_id = parameters.get_text("commands_database_id");
                let workspaces_database_id = parameters.get_text("workspaces_database_id");

                BackupCredentials::notion(NotionBackupCredentialsParameters {
                    api_key: api_key.to_string(),
                    commands_database_id: commands_database_id.to_string(),
                    workspaces_database_id: workspaces_database_id.to_string(),
                })
            }
        };

        let result =
            SaveBackupCredentialsOperation::new(SaveBackupCredentialsOperationParameters {
                save_provider: &self.storage,
                backup_provider_builder: &MockNotionBuilder {
                    storage: self.notion_storage.clone(),
                },
            })
            .execute(&backup_credentials);

        self.operation.set_result(result);
    }
}

#[test]
fn it_saves_notion_backup_credentials() {
    let mut test_case = SaveBackupCredentialsTestCase::default();

    test_case.execute_operation(table! {
        backup_provider_kind = "Notion"
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.inspect_operation_results(table! {
        [storage.backup_credentials]
        backup_provider_kind = "Notion"
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });
}
