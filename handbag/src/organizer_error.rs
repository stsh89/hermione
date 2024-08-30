use crate::{WorkspaceName, Id};
pub enum OrganizerError {
    NotFound(String),
    Fatal(String),
}

impl OrganizerError {
    pub(crate) fn fatal(description: String) -> Self {
        Self::Fatal(description)
    }

    pub(crate) fn not_found(description: String) -> Self {
        Self::NotFound(description)
    }

    pub(crate) fn workspace_not_found(workspace_id: &Id) -> Self {
        let description = format!("Workspace with id `{}` not found", workspace_id);

        Self::not_found(description)
    }

    pub(crate) fn command_not_found(workspace_name: &WorkspaceName, command_id: &Id) -> Self {
        let description = format!("Can't get command with id `{}` for `{}` workspace", command_id, workspace_name);

        Self::not_found(description)
    }
}
