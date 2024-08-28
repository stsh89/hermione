mod workspace_context;
mod workspace_form_context;
mod workspaces_context;

use ratatui::{crossterm::event::KeyCode, Frame};
pub use workspace_context::WorkspaceContext;
pub use workspace_form_context::WorkspaceFormContext;
pub use workspaces_context::WorkspacesContext;

use super::message::Message;

pub enum Context {
    Workspaces(WorkspacesContext),
    Workspace(WorkspaceContext),
    WorkspaceForm(WorkspaceFormContext),
}

impl Context {
    pub fn handle_key(&self, key_code: KeyCode) -> Option<Message> {
        match &self {
            Self::Workspaces(_) => match key_code {
                KeyCode::Char('q') => Some(Message::CloseLens),
                KeyCode::Esc => Some(Message::CloseLens),
                KeyCode::Up => Some(Message::SelectPreviousWorkspace),
                KeyCode::Down => Some(Message::SelectNextWorkspace),
                KeyCode::Enter => Some(Message::EnterWorkspace),
                KeyCode::Char('d') => Some(Message::DeleteWorkspace),
                KeyCode::Char('n') => Some(Message::EnterWorkspaceForm),
                _ => None,
            },
            Self::Workspace(_) => match key_code {
                KeyCode::Char('q') => Some(Message::CloseLens),
                KeyCode::Esc => Some(Message::ExitWorkspace),
                KeyCode::Up => Some(Message::SelectPreviousCommand),
                KeyCode::Down => Some(Message::SelectNextCommand),
                _ => None,
            },
            Self::WorkspaceForm(_) => match key_code {
                KeyCode::Esc => Some(Message::ExitWorkspaceForm),
                KeyCode::Enter => Some(Message::CreateWorkspace),
                KeyCode::Char(to_insert) => Some(Message::WorkspaceFormAddChar(to_insert)),
                KeyCode::Backspace => Some(Message::WorkspaceFormNameDeleteChar),
                KeyCode::Left => Some(Message::WorkspaceFormMoveCusorLeft),
                KeyCode::Right => Some(Message::WorkspaceFormMoveCusorRight),
                _ => None,
            },
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        match &self {
            Self::Workspaces(inner) => inner.render(frame),
            Self::Workspace(inner) => inner.render(frame),
            Self::WorkspaceForm(inner) => inner.render(frame),
        }
    }
}
