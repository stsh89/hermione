use super::{Context, ListItem, State};
use hermione_drive::ServiceFactory;
use hermione_nexus::{
    definitions::{Command, CommandId, Workspace, WorkspaceId},
    operations::{
        ExecuteCommandOperation, ListCommandsOperation, ListCommandsParameters,
        ListWorkspacesOperation, ListWorkspacesParameters,
    },
};

pub struct RunCommandOptions {
    pub no_exit: bool,
}

pub fn list_commands(state: &State, services: &ServiceFactory) -> anyhow::Result<Vec<ListItem>> {
    let Context::Commands { workspace_id } = state.context else {
        return Ok(Vec::new());
    };

    let commands = ListCommandsOperation {
        provider: &services.storage(),
    }
    .execute(ListCommandsParameters {
        page_size: None,
        page_number: None,
        program_contains: Some(&state.list.filter),
        workspace_id: Some(WorkspaceId::new(workspace_id)?),
    })?;

    Ok(commands.into_iter().map(Into::into).collect())
}

pub fn list_workspaces(state: &State, services: &ServiceFactory) -> anyhow::Result<Vec<ListItem>> {
    let workspaces = ListWorkspacesOperation {
        provider: &services.storage(),
    }
    .execute(ListWorkspacesParameters {
        name_contains: Some(&state.list.filter),
        page_number: None,
        page_size: None,
    })?;

    Ok(workspaces.into_iter().map(Into::into).collect())
}

pub fn run_command(
    state: &mut State,
    services: &ServiceFactory,
    options: RunCommandOptions,
) -> anyhow::Result<()> {
    let Context::Commands { .. } = state.context else {
        return Ok(());
    };

    if state.list.items.is_empty() {
        return Ok(());
    }

    let command_id = CommandId::new(state.list.items[state.list.cursor].id)?;

    let RunCommandOptions { no_exit } = options;

    let storage = services.storage();

    let mut system = services.system();
    system.set_no_exit(no_exit);

    ExecuteCommandOperation {
        find_command_provider: &storage,
        find_workspace_provider: &storage,
        system_provider: &system,
        track_command_provider: &storage,
        track_workspace_provider: &storage,
    }
    .execute(command_id)?;

    Ok(())
}

impl From<Workspace> for ListItem {
    fn from(value: Workspace) -> Self {
        ListItem {
            id: value.id().as_uuid(),
            text: value.name().to_string(),
        }
    }
}

impl From<Command> for ListItem {
    fn from(value: Command) -> Self {
        ListItem {
            id: value.id().as_uuid(),
            text: value.program().to_string(),
        }
    }
}
