mod core {
    pub use hermione_nexus::{
        definitions::{Command, CommandParameters},
        Error, Result,
    };
}

use ratatui::widgets::ListItem;
use uuid::Uuid;

pub struct Command {
    pub workspace_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub program: String,
}

impl<'a> From<&Command> for ListItem<'a> {
    fn from(command: &Command) -> Self {
        ListItem::new(command.program.clone())
    }
}

impl From<core::Command> for Command {
    fn from(command: core::Command) -> Self {
        Self {
            id: **command.id(),
            name: command.name().to_string(),
            program: command.program().to_string(),
            workspace_id: **command.workspace_id(),
        }
    }
}

impl TryFrom<Command> for core::Command {
    type Error = core::Error;

    fn try_from(value: Command) -> core::Result<Self> {
        let Command {
            id,
            name,
            program,
            workspace_id,
        } = value;

        let command = core::Command::new(core::CommandParameters {
            id,
            name,
            last_execute_time: None,
            program,
            workspace_id: workspace_id.into(),
        })?;

        Ok(command)
    }
}
