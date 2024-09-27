pub mod json;

use crate::types::{Command, Result, Workspace};
use hermione_memories::{
    operations::workspaces,
    types::{
        command::{self, WorkspaceScopeId},
        shared::Id,
        workspace,
    },
};
use std::str::FromStr;

pub struct Client {
    inner: json::Client,
}

impl Client {
    pub fn new(inner: json::Client) -> Self {
        Self { inner }
    }

    pub fn create_command(&self, command: Command) -> Result<()> {
        let Command {
            workspace_id,
            id: _,
            name,
            program,
        } = command;

        let command = command::Entity::new(command::NewParameters {
            name: command::Name::new(name),
            program: command::Program::new(program),
            workspace_id: Id::from_str(&workspace_id)?,
        });

        workspaces::commands::create::Operation {
            creator: &self.inner,
        }
        .execute(command)?;

        Ok(())
    }

    pub fn create_workspace(&self, workspace: Workspace) -> Result<()> {
        let Workspace {
            id: _,
            location,
            name,
        } = workspace;

        let workspace = workspace::Entity::new(workspace::NewParameters {
            name: workspace::Name::new(name),
            location: Some(workspace::Location::new(location)),
        });

        workspaces::create::Operation {
            creator: &self.inner,
        }
        .execute(workspace)?;

        Ok(())
    }

    pub fn delete_command(&self, workspace_id: &str, command_id: &str) -> Result<()> {
        workspaces::commands::delete::Operation {
            deleter: &self.inner,
        }
        .execute(WorkspaceScopeId {
            workspace_id: Id::from_str(workspace_id)?,
            command_id: Id::from_str(command_id)?,
        })?;

        Ok(())
    }

    pub fn delete_workspace(&self, workspace_id: &str) -> Result<()> {
        workspaces::delete::Operation {
            deleter: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(())
    }

    pub fn get_command(&self, workspace_id: &str, command_id: &str) -> Result<Command> {
        let command = workspaces::commands::get::Operation {
            getter: &self.inner,
        }
        .execute(WorkspaceScopeId {
            workspace_id: Id::from_str(workspace_id)?,
            command_id: Id::from_str(command_id)?,
        })?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, workspace_id: &str) -> Result<Workspace> {
        let workspace = workspaces::get::Operation {
            getter: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(workspace.into())
    }

    pub fn list_commands(&self, workspace_id: &str) -> Result<Vec<Command>> {
        let commands = workspaces::commands::list::Operation {
            lister: &self.inner,
        }
        .execute(Id::from_str(workspace_id)?)?;

        Ok(commands.into_iter().map(Into::into).collect())
    }

    pub fn list_workspaces(&self) -> Result<Vec<Workspace>> {
        let workspaces = workspaces::list::Operation {
            lister: &self.inner,
        }
        .execute()?;

        Ok(workspaces.into_iter().map(Into::into).collect())
    }

    pub fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace = load_workspace_entity(workspace)?;

        let workspace = workspaces::track_access_time::Operation {
            tracker: &self.inner,
        }
        .execute(workspace)?;

        Ok(workspace.into())
    }

    pub fn track_command_execution_time(&self, command: Command) -> Result<Command> {
        let command = load_command_entity(command)?;

        let command = workspaces::commands::track_execution_time::Operation {
            tracker: &self.inner,
        }
        .execute(command)?;

        Ok(command.into())
    }

    pub fn update_command(&self, command: Command) -> Result<Command> {
        let command = load_command_entity(command)?;

        let command = workspaces::commands::update::Operation {
            updater: &self.inner,
        }
        .execute(command)?;

        Ok(command.into())
    }

    pub fn update_workspace(&self, workspace: Workspace) -> Result<Workspace> {
        let workspace = load_workspace_entity(workspace)?;

        let workspace = workspaces::update::Operation {
            updater: &self.inner,
        }
        .execute(workspace)?;

        Ok(workspace.into())
    }
}

impl From<command::Entity> for Command {
    fn from(value: command::Entity) -> Self {
        Command {
            id: value.get_id().map(|id| id.to_string()),
            name: value.name().to_string(),
            program: value.program().to_string(),
            workspace_id: value.workspace_id().to_string(),
        }
    }
}

impl From<workspace::Entity> for Workspace {
    fn from(value: workspace::Entity) -> Self {
        Workspace {
            id: value.get_id().map(|id| id.to_string()),
            location: value
                .location()
                .map(|location| location.to_string())
                .unwrap_or_default(),
            name: value.name().to_string(),
        }
    }
}

fn load_command_entity(command: Command) -> Result<command::Entity> {
    let Command {
        workspace_id,
        id,
        name,
        program,
    } = command;

    let Some(id) = id else {
        return Err(anyhow::anyhow!("Command id is required"));
    };

    Ok(command::Entity::load(command::LoadParameters {
        last_execute_time: None,
        id: Id::from_str(&id)?,
        name: command::Name::new(name),
        program: command::Program::new(program),
        workspace_id: Id::from_str(&workspace_id)?,
    }))
}

fn load_workspace_entity(workspace: Workspace) -> Result<workspace::Entity> {
    let Workspace { id, location, name } = workspace;

    let Some(id) = id else {
        return Err(anyhow::anyhow!("Workspace id is required"));
    };

    let workspace = workspace::Entity::load(workspace::LoadParameters {
        id: Id::from_str(&id)?,
        name: workspace::Name::new(name),
        location: Some(workspace::Location::new(location)),
        last_access_time: None,
    });

    Ok(workspace)
}
