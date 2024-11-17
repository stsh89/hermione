use crate::support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage};
use anyhow::Result;
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ImportCommandsOperation, ImportCommandsOperationParameters},
};
use serde_json::{json, Value as Json};
use std::rc::Rc;

struct ImportCommandsFromNotionTestContext {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
}

impl ImportCommandsFromNotionTestContext {
    fn assert_storage_contains_command(&self, parameters: Json) {
        let id = parameters["id"].as_str().unwrap().parse().unwrap();
        let command = support::get_command(&self.storage, id);

        support::assert_command(&command, parameters);
    }

    fn execute_operation(&self) -> hermione_nexus::Result<()> {
        ImportCommandsOperation::new(ImportCommandsOperationParameters {
            backup_credentials_provider: &self.storage,
            upsert_commands_provider: &self.storage,
            backup_provider_builder: &MockNotionBuilder {
                storage: self.notion_storage.clone(),
            },
        })
        .execute(BackupProviderKind::Notion)
    }

    fn import_commands_to_notion(&self) -> Result<()> {
        self.execute_operation()?;

        Ok(())
    }

    fn with_background() -> Self {
        let context = Self {
            storage: InMemoryStorage::empty(),
            notion_storage: Rc::new(MockNotionStorage::empty()),
        };

        notion_storage_contains_command_entry(
            &context,
            json!({
                "external_id": "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
                "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ping",
                "program": "ping 1.1.1.1",
            }),
        );

        storage_contains_valid_notion_backup_credentials(
            &context,
            json!({
                "api_key": "test_api_key",
                "commands_database_id": "test_commands_database_id",
                "workspaces_database_id": "test_workspaces_database_id",
            }),
        );

        context
    }
}

fn notion_storage_contains_command_entry(
    context: &ImportCommandsFromNotionTestContext,
    parameters: Json,
) {
    support::insert_notion_command(&context.notion_storage, parameters);
}

fn storage_contains_command(context: &ImportCommandsFromNotionTestContext, parameters: Json) {
    support::insert_command(&context.storage, parameters);
}

fn storage_contains_valid_notion_backup_credentials(
    context: &ImportCommandsFromNotionTestContext,
    parameters: Json,
) {
    support::insert_notion_backup_credentials(&context.storage, parameters)
}

#[test]
fn it_restores_commands() -> Result<()> {
    let context = ImportCommandsFromNotionTestContext::with_background();

    context.import_commands_to_notion()?;

    context.assert_storage_contains_command(json!({
        "id": "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
        "name": "Ping",
        "program": "ping 1.1.1.1",
        "last_execute_time": null,
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
    }));

    Ok(())
}

#[test]
fn it_updates_existing_commands() -> Result<()> {
    let context = ImportCommandsFromNotionTestContext::with_background();

    storage_contains_command(
        &context,
        json!({
            "id": "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
            "name": "List directory files",
            "program": "Get-ChildItem .",
            "last_execute_time": "2024-11-17 22:00:00",
            "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        }),
    );

    context.import_commands_to_notion()?;

    context.assert_storage_contains_command(json!({
        "id": "51280bfc-2eea-444a-8df9-a1e7158c2c6b",
        "name": "Ping",
        "program": "ping 1.1.1.1",
        "last_execute_time": null,
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
    }));

    Ok(())
}
