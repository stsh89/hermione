mod command_form_context;
mod workspace_context;
mod workspace_form_context;
mod workspaces_context;

pub use command_form_context::{ActiveInput, CommandFormContext};
use projection::Projection;
use ratatui::Frame;
pub use workspace_context::WorkspaceContext;
pub use workspace_form_context::WorkspaceFormContext;
pub use workspaces_context::WorkspacesContext;

pub enum Context {
    Workspaces(WorkspacesContext),
    Workspace(WorkspaceContext),
    WorkspaceForm(WorkspaceFormContext),
    CommandForm(CommandFormContext),
}

impl Context {
    pub fn workspaces(projection: &Projection) -> Self {
        Context::Workspaces(WorkspacesContext::new(projection))
    }

    pub fn view(&self, frame: &mut Frame) {
        match &self {
            Self::Workspaces(inner) => inner.render(frame),
            Self::Workspace(inner) => inner.render(frame),
            Self::WorkspaceForm(inner) => inner.render(frame),
            Self::CommandForm(inner) => inner.render(frame),
        }
    }
}
