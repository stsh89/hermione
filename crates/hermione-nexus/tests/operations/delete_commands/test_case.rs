use crate::support::{self, ExistingCommand, ExistingWorkspace, InMemoryStorage};
use hermione_nexus::{
    operations::{DeleteCommandsOperation, DeleteCommandsParameters},
    Result,
};

pub struct Background {
    pub storage: InMemoryStorage,
}

pub struct ExistingStorageData<'a> {
    pub workspace: ExistingWorkspace<'a>,
    pub command: ExistingCommand<'a>,
}

pub fn assert_operation_success(operation_result: Result<()>) {
    match operation_result {
        Ok(()) => {}
        Err(error) => panic!("Delete commands operation failed with error: {}", error),
    }
}

pub fn assert_storage_does_not_contain_command(background: &Background, command_id: &str) {
    let command =
        support::maybe_get_command(&background.storage, support::parse_command_id(command_id));

    assert!(command.is_none());
}

pub fn execute_operation(
    backgournd: &Background,
    parameters: DeleteCommandsParameters,
) -> Result<()> {
    let Background { storage } = backgournd;

    DeleteCommandsOperation {
        delete_workspace_commands: storage,
    }
    .execute(parameters)
}

pub fn setup(backgournd: &Background, data: ExistingStorageData) {
    let Background { storage } = backgournd;

    let ExistingStorageData { workspace, command } = data;

    support::insert_workspace_new(storage, workspace);
    support::insert_command_new(storage, command);
}
