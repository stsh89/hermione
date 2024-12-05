mod command_model;
mod list_backup_credentials_model;
mod notion_backup_credentials_model;
mod workspace_commands_model;
mod workspace_form_model;
mod workspaces_model;

pub use command_model::*;
pub use list_backup_credentials_model::*;
pub use notion_backup_credentials_model::*;
pub use workspace_commands_model::*;
pub use workspace_form_model::*;
pub use workspaces_model::*;

use crate::coordinator::{BackupProviderKind, Command, Workspace};
use ratatui::widgets::ListItem;

impl<'a> From<&BackupProviderKind> for ListItem<'a> {
    fn from(value: &BackupProviderKind) -> Self {
        let text = match value {
            BackupProviderKind::Notion => "Notion",
        };

        ListItem::new(text)
    }
}

impl<'a> From<&Command> for ListItem<'a> {
    fn from(command: &Command) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}
