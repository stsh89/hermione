use crate::{Error, Result};
use hermione_nexus::definitions::{
    BackupCredentials, Command, CommandParameters, Workspace, WorkspaceParameters,
};
use ratatui::widgets::ListItem;
use uuid::Uuid;

pub struct BackupCredentialsKind {
    pub name: String,
}

pub struct CommandPresenter {
    pub workspace_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub program: String,
}

pub struct WorkspacePresenter {
    pub id: Uuid,
    pub location: String,
    pub name: String,
}

impl<'a> From<&BackupCredentialsKind> for ListItem<'a> {
    fn from(value: &BackupCredentialsKind) -> Self {
        ListItem::new(value.name.clone())
    }
}

impl<'a> From<&CommandPresenter> for ListItem<'a> {
    fn from(command: &CommandPresenter) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl<'a> From<&WorkspacePresenter> for ListItem<'a> {
    fn from(workspace: &WorkspacePresenter) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl From<BackupCredentials> for BackupCredentialsKind {
    fn from(value: BackupCredentials) -> Self {
        Self {
            name: value.provider_kind().as_str().to_string(),
        }
    }
}

impl From<Command> for CommandPresenter {
    fn from(command: Command) -> Self {
        Self {
            id: **command.id(),
            name: command.name().to_string(),
            program: command.program().to_string(),
            workspace_id: **command.workspace_id(),
        }
    }
}

impl From<Workspace> for WorkspacePresenter {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: **workspace.id(),
            location: workspace.location().unwrap_or_default().into(),
            name: workspace.name().to_string(),
        }
    }
}

impl TryFrom<WorkspacePresenter> for Workspace {
    type Error = Error;

    fn try_from(value: WorkspacePresenter) -> Result<Self> {
        let WorkspacePresenter { id, location, name } = value;

        let workspace = Workspace::new(WorkspaceParameters {
            id,
            name,
            location: Some(location),
            last_access_time: None,
        })?;

        Ok(workspace)
    }
}

impl TryFrom<CommandPresenter> for Command {
    type Error = Error;

    fn try_from(value: CommandPresenter) -> Result<Self> {
        let CommandPresenter {
            id,
            name,
            program,
            workspace_id,
        } = value;

        let command = Command::new(CommandParameters {
            id,
            name,
            last_execute_time: None,
            program,
            workspace_id: workspace_id.into(),
        })?;

        Ok(command)
    }
}
