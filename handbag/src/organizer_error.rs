use crate::{Id, WorkspaceName};
pub enum OrganizerError {
    NotFound(String),
}

impl OrganizerError {
    pub(crate) fn not_found(description: String) -> Self {
        Self::NotFound(description)
    }

    pub(crate) fn workspace_not_found(workspace_id: &Id) -> Self {
        let description = format!("Workspace `{}` not found", workspace_id);

        Self::not_found(description)
    }

    pub(crate) fn command_not_found(workspace_name: &WorkspaceName, command_id: &Id) -> Self {
        let description = format!(
            "Workspace `{}`: Command `{}` not found",
            workspace_name, command_id
        );

        Self::not_found(description)
    }
}
