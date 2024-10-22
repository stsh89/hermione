use hermione_coordinator::{commands::CommandDto, workspaces::WorkspaceDto};
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

impl From<CommandPresenter> for CommandDto {
    fn from(value: CommandPresenter) -> Self {
        let CommandPresenter {
            workspace_id,
            id,
            name,
            program,
        } = value;

        CommandDto {
            id,
            last_execute_time: None,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<CommandDto> for CommandPresenter {
    fn from(value: CommandDto) -> Self {
        let CommandDto {
            id,
            last_execute_time: _,
            name,
            program,
            workspace_id,
        } = value;

        CommandPresenter {
            workspace_id,
            id,
            name,
            program,
        }
    }
}

impl<'a> From<&WorkspacePresenter> for ListItem<'a> {
    fn from(workspace: &WorkspacePresenter) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl From<WorkspacePresenter> for WorkspaceDto {
    fn from(value: WorkspacePresenter) -> Self {
        let WorkspacePresenter { id, location, name } = value;

        WorkspaceDto {
            id,
            last_access_time: None,
            location: Some(location),
            name,
        }
    }
}

impl From<WorkspaceDto> for WorkspacePresenter {
    fn from(value: WorkspaceDto) -> Self {
        let WorkspaceDto {
            id,
            last_access_time: _,
            location,
            name,
        } = value;

        WorkspacePresenter {
            id,
            location: location.unwrap_or_default(),
            name,
        }
    }
}
