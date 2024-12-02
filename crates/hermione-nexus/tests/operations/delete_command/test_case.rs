use crate::support::{self, CommandFixture, InMemoryStorage, WorkspaceFixture};
use hermione_nexus::{operations::DeleteCommandOperation, Result};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: WorkspaceFixture<'a>,
    pub command: CommandFixture<'a>,
}

pub fn assert_operation_success(operation_result: Result<()>) {
    match operation_result {
        Ok(()) => {}
        Err(error) => panic!("Delete command operation failed with error: {}", error),
    }
}

pub fn assert_storage_does_not_contain_command(background: &Background, command_id: &str) {
    let command =
        support::maybe_get_command(&background.storage, support::parse_command_id(command_id));

    assert!(command.is_none());
}

pub fn execute_operation(backgournd: &Background, command_id: &str) -> Result<()> {
    let Background { storage } = backgournd;

    DeleteCommandOperation {
        find_provider: storage,
        delete_provider: storage,
    }
    .execute(support::parse_command_id(command_id))
}

pub fn setup(backgournd: &Background, data: ExistingStorageData) {
    let Background { storage } = backgournd;

    let ExistingStorageData { workspace, command } = data;

    support::insert_workspace(storage, workspace);
    support::insert_command(storage, command);
}
