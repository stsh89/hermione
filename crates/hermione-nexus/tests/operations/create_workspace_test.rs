use crate::support::{self, InMemoryStorage};
use anyhow::Result;
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
    fn assert_returned_workspace(&self, parameters: Json) {
        let workspace = self.workspace();

        support::assert_workspace(workspace, parameters);
    }

    fn assert_storage_contains_workspace(&self, parameters: Json) {
        let workspace = self.workspace();
        let workspace = support::get_workspace(&self.storage, **workspace.id());

        support::assert_workspace(&workspace, parameters);
    }

    fn create_workspace(&mut self, parameters: Json) -> Result<()> {
        let name = parameters["name"].as_str().unwrap().to_string();
        let location = parameters["location"].as_str().map(ToString::to_string);

        let workspace = CreateWorkspaceOperation {
            storage_provider: &self.storage,
        }
        .execute(CreateWorkspaceParameters { name, location })?;

        self.workspace = Some(workspace);

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
