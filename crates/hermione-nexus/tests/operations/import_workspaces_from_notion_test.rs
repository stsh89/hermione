use crate::{table, support::{self, InMemoryStorage, MockNotionBuilder, MockNotionStorage}};
use anyhow::Result;
use hermione_nexus::{
    definitions::BackupProviderKind,
    operations::{ImportWorkspacesOperation, ImportWorkspacesOperationParameters},
};
use toml::Table;
use std::rc::Rc;

struct ImportWorkspacesFromNotionTestContext {
    storage: InMemoryStorage,
    notion_storage: Rc<MockNotionStorage>,
}

impl ImportWorkspacesFromNotionTestContext {
    fn assert_storage_contains_workspace(&self, table: Table) {
        let id = table["id"].as_str().unwrap().parse().unwrap();
        let workspace = support::get_workspace(&self.storage, id);

        support::assert_workspace(&workspace, table);
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

    fn prepare() -> Self {
        Self::with_background(table! {
            [workspace_backup_exists]
            external_id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
            name = "Ironman"
            location = "/home/ironman"

            [valid_notion_backup_credentials_exist]
            api_key = "test_api_key"
            commands_database_id = "test_commands_database_id"
            workspaces_database_id = "test_workspaces_database_id"
        })
    }

    fn with_background(table: Table) -> Self {
        let ctx = ImportWorkspacesFromNotionTestContext {
            storage: InMemoryStorage::empty(),
            notion_storage: Rc::new(MockNotionStorage::empty()),
        };

        support::insert_notion_workspace(&ctx.notion_storage, &table["workspace_backup_exists"]);
        support::insert_notion_backup_credentials(&ctx.storage, &table["valid_notion_backup_credentials_exist"]);

        ctx
    }
}

// fn storage_contains_workspace(ctx: &ImportWorkspacesFromNotionTestContext, parameters: Json) {
//     support::insert_workspace(&ctx.storage, parameters);
// }

#[test]
fn it_restores_workspaces() -> Result<()> {
    let context = ImportWorkspacesFromNotionTestContext::prepare();

    context.import_workspaces_to_notion()?;

    context.assert_storage_contains_workspace(table! {
        id = "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
        name = "Ironman"
        location = "/home/ironman"
    });

    Ok(())
}

#[test]
fn it_updates_existing_workspaces() -> Result<()> {
    let context = ImportWorkspacesFromNotionTestContext::prepare();

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
