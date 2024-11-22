use crate::{
    support::{self, InMemoryStorage, MockSystem},
    Result,
};
use hermione_nexus::operations::{ExecuteProgramOperation, ExecuteProgramParameters};
use serde_json::{json, Value as Json};
use uuid::Uuid;

#[derive(Default)]
struct ExecuteProgramTestContext {
    system: MockSystem,
    storage: InMemoryStorage,
}

impl ExecuteProgramTestContext {
    fn assert_executed_program(&self, program: &str) {
        let system_program = support::get_last_executed_command(&self.system);

        assert_eq!(system_program.as_deref(), Some(program))
    }

    fn assert_system_location(&self, location: &str) {
        let system_location = support::get_system_location(&self.system);

        assert_eq!(system_location.as_deref(), Some(location));
    }

    fn execute_program(&self, parameters: Json) -> hermione_nexus::Result<()> {
        let program = parameters["program"].as_str().unwrap();
        let workspace_id: Uuid = parameters["workspace_id"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();

        ExecuteProgramOperation {
            system: &self.system,
            find_workspace: &self.storage,
        }
        .execute(ExecuteProgramParameters {
            program,
            workspace_id: workspace_id.into(),
        })
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

fn storage_contains_workspace(context: &ExecuteProgramTestContext, parameters: Json) {
    support::insert_workspace(&context.storage, parameters);
}

#[test]
fn it_executes_program() -> Result<()> {
    let context = ExecuteProgramTestContext::with_background();

    context.execute_program(json!({
        "program": "Get-ChildItem",
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    }))?;

    context.assert_executed_program("Get-ChildItem");

    Ok(())
}

#[test]
fn it_changes_system_location() -> Result<()> {
    let context = ExecuteProgramTestContext::with_background();

    context.execute_program(json!({
        "program": "Get-ChildItem",
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    }))?;

    context.assert_system_location("/home/ironman");

    Ok(())
}
