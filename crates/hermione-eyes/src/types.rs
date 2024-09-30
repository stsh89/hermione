use hermione_deeds::dtos::{command, workspace};
use ratatui::widgets::ListItem;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

pub struct Workspace {
    pub id: String,
    pub location: String,
    pub name: String,
}

pub struct Command {
    pub workspace_id: String,
    pub id: String,
    pub name: String,
    pub program: String,
}

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl<'a> From<&Command> for ListItem<'a> {
    fn from(command: &Command) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl From<Command> for command::Dto {
    fn from(value: Command) -> Self {
        let Command {
            workspace_id,
            id,
            name,
            program,
        } = value;

        command::Dto {
            id,
            last_execute_time: None,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<Workspace> for workspace::Dto {
    fn from(value: Workspace) -> Self {
        let Workspace { id, location, name } = value;

        workspace::Dto {
            id,
            last_access_time: None,
            location: Some(location),
            name,
        }
    }
}

impl From<command::Dto> for Command {
    fn from(value: command::Dto) -> Self {
        Command {
            workspace_id: value.workspace_id,
            id: value.id,
            name: value.name,
            program: value.program,
        }
    }
}

impl From<workspace::Dto> for Workspace {
    fn from(value: workspace::Dto) -> Self {
        Workspace {
            id: value.id,
            location: value.location.unwrap_or_default(),
            name: value.name,
        }
    }
}
