use crate::support::{self, InMemoryStorage};
use anyhow::Result;
use hermione_nexus::{
    definitions::Command,
    operations::{CreateCommandOperation, CreateCommandParameters},
};
use serde_json::{json, Value as Json};
use uuid::Uuid;

struct TestContext {
    storage: InMemoryStorage,
    command: Option<Command>,
}

impl TestContext {
    fn assert_returned_command(&self, parameters: Json) {
        let command = self.command();

        support::assert_command(command, parameters);
    }

    fn assert_storage_contains_command(&self, parameters: Json) {
        let command = self.command();
        let command = support::get_command(&self.storage, **command.id());

        support::assert_command(&command, parameters);
    }

    fn command(&self) -> &Command {
        self.command.as_ref().unwrap()
    }

    fn create_command(&mut self, parameters: Json) -> Result<()> {
        let name = parameters["name"].as_str().unwrap();
        let program = parameters["program"].as_str().unwrap();
        let workspace_id = parameters["workspace_id"].as_str().unwrap();

        let command = CreateCommandOperation {
            storage_provider: &self.storage,
        }
        .execute(CreateCommandParameters {
            name: name.to_string(),
            program: program.to_string(),
            workspace_id: workspace_id.parse::<Uuid>().unwrap().into(),
        })?;

        self.command = Some(command);

        Ok(())
    }

    fn with_background() -> TestContext {
        let context = TestContext {
            storage: InMemoryStorage::default(),
            command: None,
        };

        storage_contains_workspace(
            &context,
            json!({
                "id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
                "name": "Ironman",
            }),
        );

        context
    }
}

fn storage_contains_workspace(context: &TestContext, parameters: Json) {
    support::insert_workspace(&context.storage, parameters);
}

#[test]
fn it_creates_command() -> Result<()> {
    let mut context = TestContext::with_background();

    context.create_command(json!({
        "name": "Ping",
        "program": "ping 1.1.1.1",
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa",
    }))?;

    context.assert_returned_command(json!({
        "name": "Ping",
        "program": "ping 1.1.1.1",
        "last_execute_time": null,
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    }));

    context.assert_storage_contains_command(json!({
        "name": "Ping",
        "program": "ping 1.1.1.1",
        "last_execute_time": null,
        "workspace_id": "9db9a48b-f075-4518-bdd5-ec9d9b05f4fa"
    }));

    Ok(())
}
