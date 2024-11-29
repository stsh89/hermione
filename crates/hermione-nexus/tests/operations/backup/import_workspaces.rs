use crate::{
    prelude::*,
    support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage},
};
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
};
use std::rc::Rc;

#[derive(Default)]
struct ImportWorkspacesFromNotionTestCase {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
    operation: Operation<()>,
}

impl TestCase for ImportWorkspacesFromNotionTestCase {
    fn execute_operation(&mut self, parameters: Table) {
        let backup_provider_name = parameters.get_text("backup_provider_kind");
        let backup_provider_kind = support::get_backup_provider_kind(backup_provider_name);

        let backup_provider_builder = match backup_provider_kind {
            BackupProviderKind::Notion => &MockNotionBuilder {
                storage: self.notion_storage.clone(),
            },
        };

        let result = ImportWorkspacesOperation::new(ImportWorkspacesOperationParameters {
            backup_credentials_provider: &self.storage,
            upsert_workspaces_provider: &self.storage,
            backup_provider_builder,
        })
        .execute(backup_provider_kind);

        self.operation.set_result(result);
    }

    fn inspect_operation_results(&self, parameters: Table) {
        self.operation.assert_is_success();

        let storage_table = parameters.get_table("storage");
        let workspace_table = storage_table.get_table("workspace");
        let workspace_id = workspace_table.get_workspace_id("id");
        let workspace = &support::get_workspace(&self.storage, workspace_id);

        support::assert_workspace(workspace, &workspace_table);
    }

    fn setup(&mut self, parameters: Table) {
        if let Some(notion_table) = parameters.maybe_get_table("notion") {
            let notion_storage_table = notion_table.get_table("storage");
            let workspace = notion_storage_table.get_table("workspace");

            support::insert_notion_workspace(&self.notion_storage, workspace);
        }

        let storage_table = parameters.get_table("storage");
        let credentials_table = storage_table.get_table("backup_credentials");

        if let Some(credentials) = credentials_table.maybe_get_table("notion") {
            support::insert_notion_backup_credentials(&self.storage, credentials);
        }
    }
}

#[test]
fn it_restores_workspaces_from_notion() {
    let mut test_case = ImportWorkspacesFromNotionTestCase::default();

    test_case.setup(table! {
        [notion.storage.workspace]
        external_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"

        [storage.backup_credentials.notion]
        api_key = "test_api_key"
        commands_database_id = "test_commands_database_id"
        workspaces_database_id = "test_workspaces_database_id"
    });

    test_case.execute_operation(table! {
        backup_provider_kind = "Notion"
    });

    test_case.inspect_operation_results(table! {
        [storage.workspace]
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        location = "/home/ironman"
        name = "Ironman"
    });
}
