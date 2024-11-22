use crate::{
    support::{self, InMemoryStorage, MockSystem},
    Result,
};
use hermione_nexus::operations::VisitWorkspaceLocationOperation;
use serde_json::{json, Value as Json};
use uuid::Uuid;

#[derive(Default)]
struct VisitWorkspaceLocationTestContext {
    storage: InMemoryStorage,
    system: MockSystem,
}

impl VisitWorkspaceLocationTestContext {
    fn assert_working_directory(&self, location: &str) {
        let system_location = support::get_system_location(&self.system);

        assert_eq!(system_location.as_deref(), Some(location));
    }

    fn visit_workspace_location(&self, workspace_id: &str) -> hermione_nexus::Result<()> {
        let id: Uuid = workspace_id.parse().unwrap();

        VisitWorkspaceLocationOperation {
            find_workspace: &self.storage,
            system_provider: &self.system,
        }
        .execute(&id.into())
    }

    fn with_background() -> Self {
        let context = Self::default();

        storage_contains_workspace(
            &context,
            json!({
                "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ironman",
                "location": "/home/ironman",
            }),
        );

        context
    }
}

fn storage_contains_workspace(context: &VisitWorkspaceLocationTestContext, parameters: Json) {
    support::insert_workspace(&context.storage, parameters);
}

#[test]
fn it_changes_working_directory() -> Result<()> {
    let context = VisitWorkspaceLocationTestContext::with_background();

    context.visit_workspace_location("9db9a48b-f075-4518-bdd5-ec9d9b05f4fa")?;
    context.assert_working_directory("/home/ironman");

    Ok(())
}
