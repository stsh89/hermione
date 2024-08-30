mod command_execution_context;
mod command_form_context;
mod workspace_context;
mod workspace_form_context;
mod workspaces_context;

pub use command_execution_context::CommandExecutionContext;
pub use command_form_context::{ActiveInput, CommandFormContext};
use handbag::Organizer;
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
    pub fn delete_char(&mut self) {
        match self {
            Self::CommandForm(ref mut inner) => inner.delete_char(),
            Self::WorkspaceForm(ref mut inner) => inner.delete_char(),
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => {},
        }
    }

    pub fn enter_char(&mut self, char: char) {
        match self {
            Self::CommandForm(ref mut inner) => inner.enter_char(char),
            Self::WorkspaceForm(ref mut inner) => inner.enter_char(char),
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => {},
        }
    }

    pub fn is_in_editor_mode(&self) -> bool {
        match self {
            Self::WorkspaceForm(_) | Self::CommandForm(_) => true,
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => false,
        }
    }

    pub fn move_cursor_left(&mut self) {
        match self {
            Self::CommandForm(ref mut inner) => inner.move_cursor_left(),
            Self::WorkspaceForm(ref mut inner) => inner.move_cursor_left(),
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => {},
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self {
            Self::CommandForm(ref mut inner) => inner.move_cursor_right(),
            Self::WorkspaceForm(ref mut inner) => inner.move_cursor_right(),
            Self::Workspace(_) | Self::Workspaces(_) | Self::CommandExecution(_) => {},
        }
    }

    pub fn toggle_active_input(&mut self) {
        match self {
            Self::CommandForm(ref mut context) => context.toggle_active_input(),
            Self::Workspace(_)
            | Self::WorkspaceForm(_)
            | Self::Workspaces(_)
            | Self::CommandExecution(_) => {}
        }
    }

    pub fn workspaces(organizer: &Organizer) -> Self {
        Context::Workspaces(WorkspacesContext::new(organizer))
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
