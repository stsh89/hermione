use hermione_ops::{
    commands::{Command, LoadCommandParameters},
    workspaces::{LoadWorkspaceParameters, Workspace},
};
use ratatui::widgets::ListItem;

pub struct CommandPresenter {
    pub workspace_id: String,
    pub id: String,
    pub name: String,
    pub program: String,
}

pub struct WorkspacePresenter {
    pub id: String,
    pub location: String,
    pub name: String,
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

impl From<Workspace> for WorkspacePresenter {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id().map(|id| id.to_string()).unwrap_or_default(),
            location: workspace.location().unwrap_or_default().into(),
            name: workspace.name().to_string(),
        }
    }
}

impl TryFrom<WorkspacePresenter> for Workspace {
    type Error = anyhow::Error;

    fn try_from(value: WorkspacePresenter) -> anyhow::Result<Self> {
        let WorkspacePresenter { id, location, name } = value;

        Ok(Workspace::load(LoadWorkspaceParameters {
            id: id.parse()?,
            name,
            location: Some(location),
            last_access_time: None,
        }))
    }
}

impl From<Command> for CommandPresenter {
    fn from(command: Command) -> Self {
        Self {
            id: command.id().map(|id| id.to_string()).unwrap_or_default(),
            name: command.name().to_string(),
            program: command.program().to_string(),
            workspace_id: command.workspace_id().to_string(),
        }
    }
}

impl TryFrom<CommandPresenter> for Command {
    type Error = anyhow::Error;

    fn try_from(value: CommandPresenter) -> anyhow::Result<Self> {
        let CommandPresenter {
            id,
            name,
            program,
            workspace_id,
        } = value;

        Ok(Command::load(LoadCommandParameters {
            id: id.parse()?,
            name,
            last_execute_time: None,
            program,
            workspace_id: workspace_id.parse()?,
        }))
    }
}
