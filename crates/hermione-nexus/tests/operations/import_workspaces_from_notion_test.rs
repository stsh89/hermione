use crate::support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage};
use anyhow::Result;
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
};
use serde_json::json;
use serde_json::Value as Json;
use std::rc::Rc;

struct ImportWorkspacesFromNotionTestContext {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
}

impl ImportWorkspacesFromNotionTestContext {
    fn assert_storage_contains_workspace(&self, parameters: Json) {
        let id = parameters["id"].as_str().unwrap().parse().unwrap();
        let workspace = support::get_workspace(&self.storage, id);

        support::assert_workspace(&workspace, parameters);
    }

    fn import_workspaces_to_notion(&self) -> Result<()> {
        ImportWorkspacesOperation::new(ImportWorkspacesOperationParameters {
            backup_credentials_provider: &self.storage,
            upsert_workspaces_provider: &self.storage,
            backup_provider_builder: &MockNotionBuilder {
                storage: self.notion_storage.clone(),
            },
        })
        .execute(BackupProviderKind::Notion)?;

        Ok(())
    }

    fn with_background() -> Self {
        let ctx = ImportWorkspacesFromNotionTestContext {
            storage: InMemoryStorage::empty(),
            notion_storage: Rc::new(MockNotionStorage::empty()),
        };

        notion_storage_contains_workspace_entry(
            &ctx,
            json!({
                "external_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ironman",
                "location": "/home/ironman"
            }),
        );

        storage_contains_valid_notion_backup_credentials(
            &ctx,
            json!({
                "api_key": "test_api_key",
                "commands_database_id": "test_commands_database_id",
                "workspaces_database_id": "test_workspaces_database_id"
            }),
        );

        ctx
    }
}

fn storage_contains_valid_notion_backup_credentials(
    ctx: &ImportWorkspacesFromNotionTestContext,
    parameters: Json,
) {
    support::insert_notion_backup_credentials(&ctx.storage, parameters)
}

fn storage_contains_workspace(ctx: &ImportWorkspacesFromNotionTestContext, parameters: Json) {
    support::insert_workspace(&ctx.storage, parameters);
}

fn notion_storage_contains_workspace_entry(
    ctx: &ImportWorkspacesFromNotionTestContext,
    parameters: Json,
) {
    support::insert_notion_workspace(&ctx.notion_storage, parameters);
}

#[test]
fn it_restores_workspaces() -> Result<()> {
    let context = ImportWorkspacesFromNotionTestContext::with_background();

    context.import_workspaces_to_notion()?;

    context.assert_storage_contains_workspace(json!({
        "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    Ok(())
}

#[test]
fn it_updates_existing_workspaces() -> Result<()> {
    let context = ImportWorkspacesFromNotionTestContext::with_background();

    storage_contains_workspace(
        &context,
        json!({
            "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
            "name": "Avenger",
            "location": "/home/avenger",
            "last_access_time": "2024-11-17 20:00:00"
        }),
    );

    context.import_workspaces_to_notion()?;

    context.assert_storage_contains_workspace(json!({
        "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    Ok(())
}
