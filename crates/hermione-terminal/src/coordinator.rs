use crate::{CommandPresenter, Result, WorkspacePresenter};
use hermione_coordinator::{
    commands::{CommandsClient, ListCommandsWithinWorkspaceInput},
    workspaces::{ListWorkspacesInput, WorkspacesClient},
    Connection,
};
use std::path::Path;

pub struct Coordinator {
    workspaces: WorkspacesCoordinator,
    commands: CommandsCoordinator,
}

pub struct WorkspacesCoordinator {
    client: WorkspacesClient,
}

pub struct CommandsCoordinator {
    client: CommandsClient,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        let connection = Connection::new(app_path)?;

        Ok(Self {
            workspaces: WorkspacesCoordinator::new(&connection)?,
            commands: CommandsCoordinator::new(&connection)?,
        })
    }

    pub fn workspaces(&self) -> &WorkspacesCoordinator {
        &self.workspaces
    }

    pub fn commands(&self) -> &CommandsCoordinator {
        &self.commands
    }
}

pub struct ListCommandsWithinWorkspaceFilter<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: &'a str,
}

impl CommandsCoordinator {
    fn new(connection: &Connection) -> Result<Self> {
        Ok(Self {
            client: CommandsClient::new(connection)?,
        })
    }

    pub fn create(&self, command: CommandPresenter) -> Result<CommandPresenter> {
        let data = self.client.create_command(command.into())?;

        Ok(data.into())
    }

    pub fn delete(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        self.client
            .delete_command_from_workspace(workspace_id, command_id)?;

        Ok(())
    }

    pub fn get(&self, workspace_id: &str, command_id: &str) -> Result<CommandPresenter> {
        let command = self
            .client
            .get_command_from_workspace(workspace_id, command_id)?;

        Ok(command.into())
    }

    pub fn list(
        &self,
        parameters: ListCommandsWithinWorkspaceFilter,
    ) -> Result<Vec<CommandPresenter>> {
        let ListCommandsWithinWorkspaceFilter {
            workspace_id,
            program_contains,
            page_number,
            page_size,
        } = parameters;

        let commands =
            self.client
                .list_commands_within_workspace(ListCommandsWithinWorkspaceInput {
                    workspace_id,
                    program_contains,
                    page_number,
                    page_size,
                })?;

        Ok(commands.into_iter().map(Into::into).collect())
    }

    pub fn track_execution_time(&self, command: CommandPresenter) -> Result<CommandPresenter> {
        let command = self
            .client
            .track_command_execution_time(&command.workspace_id, &command.id)?;

        Ok(command.into())
    }

    pub fn update(&self, command: CommandPresenter) -> Result<CommandPresenter> {
        let command = self.client.update_command(command.into())?;

        Ok(command.into())
    }
}

pub struct ListWorkspacesFilter<'a> {
    pub name_contains: &'a str,
    pub page_number: u32,
    pub page_size: u32,
}

impl WorkspacesCoordinator {
    fn new(connection: &Connection) -> Result<Self> {
        Ok(Self {
            client: WorkspacesClient::new(connection)?,
        })
    }

    pub fn create(&self, workspace: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let data = self.client.create_workspace(workspace.into())?;

        Ok(data.into())
    }

    pub fn delete(&self, workspace_id: &str) -> Result<()> {
        self.client.delete_workspace(workspace_id)?;

        Ok(())
    }

    pub fn get(&self, workspace_id: &str) -> Result<WorkspacePresenter> {
        let workspace = self.client.get_workspace(workspace_id)?;

        Ok(workspace.into())
    }

    pub fn list(&self, parameters: ListWorkspacesFilter) -> Result<Vec<WorkspacePresenter>> {
        let ListWorkspacesFilter {
            name_contains,
            page_number,
            page_size,
        } = parameters;

        let workspaces = self.client.list_workspaces(ListWorkspacesInput {
            name_contains,
            page_number,
            page_size,
        })?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn track_access_time(&self, workspace: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let workspace = self.client.track_workspace_access_time(&workspace.id)?;

        Ok(workspace.into())
    }

    pub fn update(&self, workspace: WorkspacePresenter) -> Result<WorkspacePresenter> {
        let workspace = self.client.update_workspace(workspace.into())?;

        Ok(workspace.into())
    }
}
