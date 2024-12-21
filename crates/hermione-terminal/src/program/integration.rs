use super::{Context, ListItem, State};
use hermione_drive::{NotionBackupBuilder, ServiceFactory};
use hermione_nexus::{
    definitions::{
        BackupCredentials, BackupProviderKind, Command, CommandId,
        NotionBackupCredentialsParameters, Workspace, WorkspaceId,
    },
    operations::{
        CommandsDeleteAttribute, CreateCommandOperation, CreateCommandParameters,
        CreateWorkspaceOperation, CreateWorkspaceParameters, DeleteBackupCredentialsOperation,
        DeleteCommandOperation, DeleteCommandsOperation, DeleteCommandsParameters,
        DeleteWorkspaceOperation, ExecuteCommandOperation, GetCommandOperation,
        GetWorkspaceOperation, ListBackupCredentialsOperation, ListCommandsOperation,
        ListCommandsParameters, ListWorkspacesOperation, ListWorkspacesParameters,
        SaveBackupCredentialsOperation, SaveBackupCredentialsOperationParameters,
        UpdateCommandOperation, UpdateCommandParameters, UpdateWorkspaceOperation,
        UpdateWorkspaceParameters,
    },
};

pub struct RunCommandOptions {
    pub no_exit: bool,
}

pub fn get_command(
    state: &mut State,
    services: &ServiceFactory,
) -> anyhow::Result<Option<Command>> {
    let Context::Commands { .. } = state.context else {
        return Ok(None);
    };

    if state.list.items.is_empty() {
        return Ok(None);
    }

    let id = state.list.items[state.list.cursor].id;

    let command = GetCommandOperation {
        provider: &services.storage(),
    }
    .execute(CommandId::new(id)?)?;

    Ok(Some(command))
}

pub fn get_notion_backup_credentials(
    services: &ServiceFactory,
) -> anyhow::Result<Option<BackupCredentials>> {
    let backup_credentials = list_backup_credentials(services)?;

    let notion_backup_credentials = backup_credentials
        .into_iter()
        .find(|credentials| matches!(credentials, BackupCredentials::Notion(..)));

    Ok(notion_backup_credentials)
}

pub fn get_workspace(
    state: &mut State,
    services: &ServiceFactory,
) -> anyhow::Result<Option<Workspace>> {
    let Context::Workspaces = state.context else {
        return Ok(None);
    };

    if state.list.items.is_empty() {
        return Ok(None);
    }

    let id = state.list.items[state.list.cursor].id;

    let workspace = GetWorkspaceOperation {
        provider: &services.storage(),
    }
    .execute(WorkspaceId::new(id)?)?;

    Ok(Some(workspace))
}

pub fn save_command(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    let Context::CommandForm {
        command_id,
        workspace_id,
    } = state.context
    else {
        return Ok(());
    };

    let storage = services.storage();

    let name = state.form.inputs[0].clone();
    let program = state.form.inputs[1].clone();
    let workspace_id = WorkspaceId::new(workspace_id)?;

    if let Some(id) = command_id {
        UpdateCommandOperation {
            find_command_provider: &storage,
            update_command_provider: &storage,
        }
        .execute(UpdateCommandParameters {
            id: CommandId::new(id)?,
            program,
            name,
        })?;
    } else {
        CreateCommandOperation {
            storage_provider: &storage,
        }
        .execute(CreateCommandParameters {
            name,
            program,
            workspace_id,
        })?;
    }

    Ok(())
}

pub fn save_notion_backup_credentials(
    state: &mut State,
    services: &ServiceFactory,
) -> anyhow::Result<()> {
    let Context::Notion = state.context else {
        return Ok(());
    };

    let is_empty_form = state.form.inputs.iter().all(|input| input.is_empty());
    let storage = services.storage();

    if is_empty_form {
        DeleteBackupCredentialsOperation {
            find_provider: &storage,
            delete_provider: &storage,
        }
        .execute(BackupProviderKind::Notion)?;
    } else {
        let credentials = BackupCredentials::notion(NotionBackupCredentialsParameters {
            api_key: state.form.inputs[0].clone(),
            commands_database_id: state.form.inputs[1].clone(),
            workspaces_database_id: state.form.inputs[2].clone(),
        });

        let backup_provider = NotionBackupBuilder::default();

        SaveBackupCredentialsOperation::new(SaveBackupCredentialsOperationParameters {
            save_provider: &storage,
            backup_provider_builder: &backup_provider,
        })
        .execute(&credentials)?;
    }

    Ok(())
}

pub fn save_workspace(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    let Context::WorkspaceForm { workspace_id } = state.context else {
        return Ok(());
    };

    let storage = services.storage();

    let name = state.form.inputs[0].clone();
    let location = state.form.inputs[1].clone();

    if let Some(id) = workspace_id {
        UpdateWorkspaceOperation {
            find_workspace_provider: &storage,
            update_workspace_provider: &storage,
        }
        .execute(UpdateWorkspaceParameters {
            id: WorkspaceId::new(id)?,
            location: Some(location),
            name,
        })?;
    } else {
        CreateWorkspaceOperation {
            storage_provider: &storage,
        }
        .execute(CreateWorkspaceParameters {
            name,
            location: Some(location),
        })?;
    }

    Ok(())
}

pub fn delete_command(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    let Context::Commands { .. } = state.context else {
        return Ok(());
    };

    if state.list.items.is_empty() {
        return Ok(());
    }

    let id = CommandId::new(state.list.items[state.list.cursor].id)?;
    let storage = services.storage();

    DeleteCommandOperation {
        find_provider: &storage,
        delete_provider: &storage,
    }
    .execute(id)?;

    Ok(())
}

pub fn delete_workspace(state: &mut State, services: &ServiceFactory) -> anyhow::Result<()> {
    let Context::Workspaces = state.context else {
        return Ok(());
    };

    if state.list.items.is_empty() {
        return Ok(());
    }

    let workspace_id = state.list.items[state.list.cursor].id;
    let id = WorkspaceId::new(workspace_id)?;
    let storage = services.storage();

    DeleteCommandsOperation {
        delete_workspace_commands: &storage,
    }
    .execute(DeleteCommandsParameters {
        delete_attribute: CommandsDeleteAttribute::WorkspaceId(id),
    })?;

    DeleteWorkspaceOperation {
        find_workspace_provider: &storage,
        delete_workspace_provider: &storage,
    }
    .execute(id)?;

    Ok(())
}

pub fn list_commands(state: &State, services: &ServiceFactory) -> anyhow::Result<Vec<ListItem>> {
    let workspace_id = match state.context {
        Context::Workspaces => {
            if state.list.items.is_empty() {
                None
            } else {
                Some(state.list.items[state.list.cursor].id)
            }
        }
        Context::WorkspaceForm { workspace_id } => workspace_id,
        Context::Commands { workspace_id } => Some(workspace_id),
        Context::CommandForm { workspace_id, .. } => Some(workspace_id),
        Context::Notion => None,
    };

    let Some(workspace_id) = workspace_id else {
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

fn list_backup_credentials(services: &ServiceFactory) -> anyhow::Result<Vec<BackupCredentials>> {
    let backup_credentials = ListBackupCredentialsOperation {
        provider: &services.storage(),
    }
    .execute()?;

    Ok(backup_credentials)
}
