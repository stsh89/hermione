use hermione_drive::Engine;
use hermione_nexus::operations::{ExecuteProgramOperation, ExecuteProgramParameters};

pub fn run() -> anyhow::Result<()> {
    let Engine {
        service_factory,
        logs_worker_guard: _logs_worker_guard,
    } = hermione_drive::start()?;

    let mut system = service_factory.system();
    system.set_no_exit(false);

    ExecuteProgramOperation {
        system: &system,
        find_workspace: &service_factory.storage(),
    }
    .execute(ExecuteProgramParameters {
        program: "cargo install --git https://github.com/stsh89/hermione.git hermione-terminal",
        workspace_id: None,
    })?;

    Ok(())
}
