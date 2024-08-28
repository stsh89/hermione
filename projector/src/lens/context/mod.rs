mod command_form_context;
mod workspace_context;
mod workspace_form_context;
mod workspaces_context;

pub use command_form_context::{ActiveInput, CommandFormContext};
use ratatui::{crossterm::event::KeyCode, Frame};
pub use workspace_context::WorkspaceContext;
pub use workspace_form_context::WorkspaceFormContext;
pub use workspaces_context::WorkspacesContext;

use super::message::Message;

pub enum Context {
    Workspaces(WorkspacesContext),
    Workspace(WorkspaceContext),
    WorkspaceForm(WorkspaceFormContext),
    CommandForm(CommandFormContext),
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
                KeyCode::Char('n') => Some(Message::EnterCommandForm),
                _ => None,
            },
            Self::WorkspaceForm(_) => match key_code {
                KeyCode::Esc => Some(Message::ExitWorkspaceForm),
                KeyCode::Enter => Some(Message::CreateWorkspace),
                KeyCode::Char(to_insert) => Some(Message::InputChar(to_insert)),
                KeyCode::Backspace => Some(Message::DeleteChar),
                KeyCode::Left => Some(Message::MoveCusorLeft),
                KeyCode::Right => Some(Message::MoveCusorRight),
                _ => None,
            },
            Self::CommandForm(_) => match key_code {
                KeyCode::Esc => Some(Message::ExitCommandForm),
                KeyCode::Enter => Some(Message::CreateCommand),
                KeyCode::Char(to_insert) => Some(Message::InputChar(to_insert)),
                KeyCode::Backspace => Some(Message::DeleteChar),
                KeyCode::Left => Some(Message::MoveCusorLeft),
                KeyCode::Right => Some(Message::MoveCusorRight),
                KeyCode::Tab => Some(Message::ToggleActiveInput),
                _ => None,
            },
        }
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
