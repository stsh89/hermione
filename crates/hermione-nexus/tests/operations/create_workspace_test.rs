use crate::support::{self, InMemoryStorage};
use anyhow::Result;
use chrono::NaiveDateTime;
use hermione_nexus::{
    definitions::Workspace,
    operations::{CreateWorkspaceOperation, CreateWorkspaceParameters},
};
use serde_json::{json, Value as Json};

struct TestContext {
    storage: InMemoryStorage,
    workspace: Option<Workspace>,
}

impl TestContext {
    fn assert_workspace(&self, workspace: &Workspace, parameters: Json) {
        let name = parameters["name"].as_str().unwrap().to_string();
        let location = parameters["location"].as_str();
        let last_access_time = parameters["last_access_time"].as_str().map(|value| {
            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc()
        });

        assert_eq!(workspace.name(), name);
        assert_eq!(workspace.location(), location);
        assert_eq!(workspace.last_access_time(), last_access_time.as_ref());
    }

    fn assert_returned_workspace(&self, parameters: Json) {
        let workspace = self.workspace();

        self.assert_workspace(workspace, parameters);
    }

    fn assert_storage_contains_workspace(&self, parameters: Json) {
        let workspace = self.workspace();
        let workspace = support::get_workspace(&self.storage, **workspace.id());

        self.assert_workspace(&workspace, parameters);
    }

    fn create_workspace(&mut self, parameters: Json) -> Result<()> {
        let name = parameters["name"].as_str().unwrap().to_string();
        let location = parameters["location"].as_str().map(ToString::to_string);

        self.workspace = Some(
            CreateWorkspaceOperation {
                storage_provider: &self.storage,
            }
            .execute(CreateWorkspaceParameters { name, location })?,
        );

        Ok(())
    }

    fn workspace(&self) -> &Workspace {
        self.workspace.as_ref().unwrap()
    }
}

#[test]
fn it_creates_workspace() -> Result<()> {
    let mut context = TestContext {
        storage: InMemoryStorage::empty(),
        workspace: None,
    };

    context.create_workspace(json!({
        "name": "Ironman",
        "location": "/home/ironman"
    }))?;

    context.assert_returned_workspace(json!({
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    context.assert_storage_contains_workspace(json!({
        "name": "Ironman",
        "location": "/home/ironman",
        "last_access_time": null
    }));

    Ok(())
}
