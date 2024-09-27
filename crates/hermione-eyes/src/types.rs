use hermione_deeds::types::{command, workspace};
use ratatui::widgets::ListItem;

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

impl From<Command> for command::Data {
    fn from(value: Command) -> Self {
        let Command {
            workspace_id,
            id,
            name,
            program,
        } = value;

        command::Data {
            id,
            last_execute_time: None,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<Workspace> for workspace::Data {
    fn from(value: Workspace) -> Self {
        let Workspace { id, location, name } = value;

        workspace::Data {
            id,
            last_access_time: None,
            location: Some(location),
            name,
        }
    }
}

impl From<command::Data> for Command {
    fn from(value: command::Data) -> Self {
        Command {
            workspace_id: value.workspace_id,
            id: value.id,
            name: value.name,
            program: value.program,
        }
    }
}

impl From<workspace::Data> for Workspace {
    fn from(value: workspace::Data) -> Self {
        Workspace {
            id: value.id,
            location: value.location.unwrap_or_default(),
            name: value.name,
        }
    }
}
