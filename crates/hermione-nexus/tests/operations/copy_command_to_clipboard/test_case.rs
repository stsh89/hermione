use crate::support::{self, ExistingCommand, ExistingWorkspace, InMemoryStorage, MockSystem};
use hermione_nexus::{operations::CopyCommandToClipboardOperation, Error};

pub struct Background {
    pub storage: InMemoryStorage,
    pub system: MockSystem,
}

pub struct BackgroundContext<'a> {
    pub command: ExistingCommand<'a>,
    pub workspace: ExistingWorkspace<'a>,
}

pub fn assert_clipboard_content(backgound: &Background, expected: &str) {
    let Background { system, .. } = backgound;

    support::assert_clipboard_content(system, expected);
}

pub fn assert_operation_success(result: Result<(), Error>) {
    assert!(result.is_ok());
}

pub fn execute_operation(backgound: &Background, command_id: &str) -> Result<(), Error> {
    let Background { storage, system } = backgound;

    CopyCommandToClipboardOperation {
        clipboard_provider: system,
        storage_provider: storage,
    }
    .execute(support::parse_command_id(command_id))
}

pub fn setup(backgound: &Background, context: BackgroundContext) {
    let Background { storage, system: _ } = backgound;

    let BackgroundContext { command, workspace } = context;

    support::insert_workspace_new(storage, workspace);
    support::insert_command_new(storage, command);
}
