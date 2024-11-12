mod core {
    pub use hermione_nexus::{
        definitions::{Workspace, WorkspaceParameters},
        Error, Result,
    };
}

use ratatui::widgets::ListItem;
use uuid::Uuid;

impl<'a> From<&Workspace> for ListItem<'a> {
    fn from(workspace: &Workspace) -> Self {
        ListItem::new(workspace.name.clone())
    }
}

impl From<core::Workspace> for Workspace {
    fn from(workspace: core::Workspace) -> Self {
        Self {
            id: **workspace.id(),
            location: workspace.location().unwrap_or_default().into(),
            name: workspace.name().to_string(),
        }
    }
}

impl TryFrom<Workspace> for core::Workspace {
    type Error = core::Error;

    fn try_from(value: Workspace) -> core::Result<Self> {
        let Workspace { id, location, name } = value;

        let workspace = core::Workspace::new(core::WorkspaceParameters {
            id,
            name,
            location: Some(location),
            last_access_time: None,
        })?;

        Ok(workspace)
    }
}
