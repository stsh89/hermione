use crate::{
    presenters::{command::Presenter as Command, workspace::Presenter as Workspace},
    Result,
};
use hermione_deeds::workspaces::{self, commands::Operations as _, Operations};
use std::path::Path;

pub struct Client {
    workspaces: workspaces::Client,
    commands: workspaces::commands::Client,
}

impl Client {
    pub fn new(path: &Path) -> Result<Self> {
        let commands_path = path.join("commands.json");
        let workspaces_path = path.join("workspaces.json");

        Ok(Self {
            workspaces: workspaces::Client::new(workspaces_path)?,
            commands: workspaces::commands::Client::new(commands_path)?,
        })
    }

    pub fn create_command(&self, command: Command) -> Result<Command> {
        let data = self.commands.create(command.into())?;

        Ok(data.into())
    }

    pub fn create_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let data = self.workspaces.create(workspace.into())?;

        Ok(data.into())
    }

    pub fn delete_command(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        self.commands.delete(workspace_id, command_id)
    }

    pub fn delete_workspace(&self, workspace_id: &str) -> Result<()> {
        self.workspaces.delete(workspace_id)
    }

    pub fn get_command(&self, workspace_id: &str, command_id: &str) -> Result<Command> {
        let command = self.commands.get(workspace_id, command_id)?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, workspace_id: &str) -> Result<Workspace> {
        let workspace = self.workspaces.get(workspace_id)?;

        Ok(workspace.into())
    }

    pub fn get_default_workspace(&self) -> Result<Workspace> {
        let workspace = self.workspaces.get("")?;

        Ok(workspace.into())
    }

    pub fn list_commands(&self, workspace_id: &str) -> Result<Vec<Command>> {
        let commands = self.commands.list(workspace_id)?;

        Ok(commands.into_iter().map(Into::into).collect())
    }

    pub fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        let workspaces = self.workspaces.list()?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace = self.workspaces.track_access_time(&workspace.id)?;

        Ok(workspace.into())
    }

    pub fn track_command_execution_time(&self, command: Command) -> Result<Command> {
        let command = self
            .commands
            .track_execution_time(&command.workspace_id, &command.id)?;

        Ok(command.into())
    }

    pub fn update_command(&self, command: Command) -> Result<Command> {
        let command = self.commands.update(command.into())?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace = self.workspaces.update(workspace.into())?;

        Ok(workspace.into())
    }
}
