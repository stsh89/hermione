use crate::support::{self, ExpectedCommand, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{
    definitions::Command,
    operations::{CreateCommandOperation, CreateCommandParameters},
    Error,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub fn assert_returned_command(command: Command, expected: ExpectedCommand) {
    support::assert_command(command, expected);
}

pub fn assert_storage_contains_command(background: &Background, expected: ExpectedCommand) {
    let command = support::get_command(&background.storage, expected.id());

    support::assert_command(command, expected);
}

pub fn execute_operation(
    background: &Background,
    parameters: CreateCommandParameters,
) -> Result<Command, Error> {
    let Background { storage } = background;

    CreateCommandOperation {
        storage_provider: storage,
    }
    .execute(parameters)
}

pub fn setup(backgound: &Background, workspace: WorkspaceFixture) {
    let Background { storage } = backgound;

    support::insert_workspace(storage, workspace);
}
