use crate::{
    support::{self, MockSystem},
    Result,
};
use hermione_nexus::operations::{ExecuteProgramOperation, ExecuteProgramParameters};
use serde_json::{json, Value as Json};

#[derive(Default)]
struct ExecuteProgramTestContext {
    system: MockSystem,
}

impl ExecuteProgramTestContext {
    fn assert_executed_program(&self, program: &str) {
        let system_program = support::get_last_executed_command(&self.system);

        assert_eq!(system_program.as_deref(), Some(program))
    }

    fn execute_program(&self, parameters: Json) -> hermione_nexus::Result<()> {
        let program = parameters["program"].as_str().unwrap();

        ExecuteProgramOperation {
            system: &self.system,
        }
        .execute(ExecuteProgramParameters { program })
    }
}

#[test]
fn it_executes_program() -> Result<()> {
    let context = ExecuteProgramTestContext::default();

    context.execute_program(json!({
        "program": "Get-ChildItem"
    }))?;

    context.assert_executed_program("Get-ChildItem");

    Ok(())
}
