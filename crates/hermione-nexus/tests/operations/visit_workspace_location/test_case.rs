use crate::support::{self, ExistingWorkspace, InMemoryStorage, MockSystem};
use hermione_nexus::{operations::VisitWorkspaceLocationOperation, Error};

pub struct Background {
    pub storage: InMemoryStorage,
    pub system: MockSystem,
}

pub fn assert_operation_success(operation_result: Result<(), Error>) {
    assert!(operation_result.is_ok());
}

pub fn assert_system_location_changed(background: &Background, expected: &str) {
    support::assert_system_location(&background.system, expected);
}

pub fn setup(background: &Background, workspace: ExistingWorkspace) {
    support::insert_workspace_new(&background.storage, workspace);
}

pub fn execute_operation(background: &Background, workspace_id: &str) -> Result<(), Error> {
    let Background { storage, system } = background;

    VisitWorkspaceLocationOperation {
        find_workspace: storage,
        system_provider: system,
    }
    .execute(support::parse_workspace_id(workspace_id))
}
