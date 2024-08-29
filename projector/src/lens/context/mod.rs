mod command_execution_context;
mod command_form_context;
mod workspace_context;
mod workspace_form_context;
mod workspaces_context;

pub use command_execution_context::CommandExecutionContext;
pub use command_form_context::{ActiveInput, CommandFormContext};
use projection::Projection;
use ratatui::Frame;
pub use workspace_context::WorkspaceContext;
pub use workspace_form_context::WorkspaceFormContext;
pub use workspaces_context::WorkspacesContext;

pub enum Context {
    CommandForm(CommandFormContext),
    Workspace(WorkspaceContext),
    WorkspaceForm(WorkspaceFormContext),
    Workspaces(WorkspacesContext),
    CommandExecution(CommandExecutionContext),
}

impl Context {
    pub fn is_editor_mode(&self) -> bool {
        match self {
            Self::WorkspaceForm(_) | Self::CommandForm(_) => true,
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => false,
        }
    }

    pub fn workspaces(projection: &Projection) -> Self {
        Context::Workspaces(WorkspacesContext::new(projection))
    }

    pub fn view(&self, frame: &mut Frame) {
        match &self {
            Self::CommandExecution(inner) => inner.render(frame),
            Self::CommandForm(inner) => inner.render(frame),
            Self::Workspace(inner) => inner.render(frame),
            Self::WorkspaceForm(inner) => inner.render(frame),
            Self::Workspaces(inner) => inner.render(frame),
        }
    }
}
