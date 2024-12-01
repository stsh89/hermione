use hermione_nexus::{
    definitions::Command, operations::{CreateCommandOperation, CreateCommandParameters}, Error
};
use crate::support::{self, ExistingWorkspace, ExpectedCommand, InMemoryStorage};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_returned_command(command: Command, expected: ExpectedCommand) {
    support::assert_command_new(command, expected);
}

pub fn assert_storage_contains_command(background: &Background, expected: ExpectedCommand) {
    let command = support::get_command(&background.storage, expected.id());

    support::assert_command_new(command, expected);
}

pub fn execute_operation(background: &Background, parameters: CreateCommandParameters) -> Result<Command, Error>{
    let Background { storage } = background;

    CreateCommandOperation {
        storage_provider: storage,
    }
    .execute(parameters)
}

pub fn setup(backgound: &Background, workspace: ExistingWorkspace) {
    let Background { storage } = backgound;

    support::insert_workspace_new(storage, workspace);
}
