use hermione_coordinator::{CommandDto, WorkspaceDto};
use ratatui::widgets::ListItem;

pub struct Command {
    pub workspace_id: String,
    pub id: String,
    pub name: String,
    pub program: String,
}

pub struct Workspace {
    pub id: String,
    pub location: String,
    pub name: String,
}

impl<'a> From<&Command> for ListItem<'a> {
    fn from(command: &Command) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl From<Command> for CommandDto {
    fn from(value: Command) -> Self {
        let Command {
            workspace_id,
            id,
            name,
            program,
        } = value;

        CommandDto {
            id,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<CommandDto> for Command {
    fn from(value: CommandDto) -> Self {
        let CommandDto {
            id,
            name,
            program,
            workspace_id,
        } = value;

        Command {
            workspace_id,
            id,
            name,
            program,
        }
    }
}

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl From<Workspace> for WorkspaceDto {
    fn from(value: Workspace) -> Self {
        let Workspace { id, location, name } = value;

        WorkspaceDto {
            id,
            location: Some(location),
            name,
        }
    }
}

impl From<WorkspaceDto> for Workspace {
    fn from(value: WorkspaceDto) -> Self {
        let WorkspaceDto { id, location, name } = value;

        Workspace {
            id,
            location: location.unwrap_or_default(),
            name,
        }
    }
}
