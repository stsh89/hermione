use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage},
};
use hermione_nexus::{
    definitions::{BackupCredentials, NotionBackupCredentialsParameters},
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
    fn assert_operation_failure(&self, parameters: Table) {
        self.operation.assert_failure();

        let error_message = self.operation.result().as_ref().unwrap_err().to_string();

        assert_eq!(error_message, parameters.get_text("error_message"));
    }

    fn assert_operation_success(&self, parameters: Table) {
        self.operation.assert_success();

        let credentials_table = parameters
            .get_table("storage")
            .get_table("backup_credentials");
        let backup_provider_kind = credentials_table.get_text("kind");

        match backup_provider_kind {
            "Notion" => {
                let backup_credentials = support::get_notion_backup_credentials(&self.storage);

                support::assert_notion_backup_credentials(&backup_credentials, credentials_table);
            }
            _ => unreachable!(),
        };
    }

    fn execute_operation(&mut self, parameters: Table) {
        let backup_provider_kind = parameters.get_text("kind");

        let backup_credentials = match backup_provider_kind {
            "Notion" => {
                let api_key = parameters.get_text("api_key");
                let commands_database_id = parameters.get_text("commands_database_id");
                let workspaces_database_id = parameters.get_text("workspaces_database_id");

                BackupCredentials::notion(NotionBackupCredentialsParameters {
                    api_key: api_key.to_string(),
                    commands_database_id: commands_database_id.to_string(),
                    workspaces_database_id: workspaces_database_id.to_string(),
                })
            }
            _ => unreachable!(),
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
        kind = "Notion"
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.assert_operation_success(table! {
        [storage.backup_credentials]
        kind = "Notion"
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });
}

#[test]
fn it_returns_verification_error_for_invalid_notion_backup_credentials() {
    let mut context = SaveBackupCredentialsTestCase::default();

    context.execute_operation(table! {
        kind = "Notion"
        api_key = "fake_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    context.assert_operation_failure(table! {
        error_message = "Invalid API key"
    });
}
