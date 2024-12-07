use super::{state::ModelState, Message};
use crate::coordinator::WorkspaceId;

pub enum Action {
    ClearInput,
    DeleteChar,
    EnterChar(char),
    GoToWorkspacesList,
    MoveCusorLeft,
    MoveCusorRight,
    GoToCreateWorkspace,
    GoToUpdateWorkspace(WorkspaceId),
    SelectNextInput,
}

pub fn dispatch(action: Action, state: &mut ModelState) {
    match action {
        Action::ClearInput => state.clear_input(),
        Action::DeleteChar => state.delete_char(),
        Action::EnterChar(c) => state.enter_char(c),
        Action::GoToWorkspacesList => state.set_redirect_to_list_workspaces(),
        Action::MoveCusorLeft => state.move_cursor_left(),
        Action::MoveCusorRight => state.move_cursor_right(),
        Action::SelectNextInput => state.select_next_input(),
        Action::GoToCreateWorkspace => state.set_redirect_to_create_workspace(),
        Action::GoToUpdateWorkspace(id) => state.set_redirect_to_update_workspace(id),
    }
}

pub fn maybe_create_action(message: Message, state: &ModelState) -> Option<Action> {
    match message {
        Message::Cancel => {
            return Some(Action::GoToWorkspacesList);
        }
        Message::DeleteAllChars => {
            return Some(Action::ClearInput);
        }
        Message::DeleteChar => {
            return Some(Action::DeleteChar);
        }
        Message::EnterChar(c) => {
            return Some(Action::EnterChar(c));
        }
        Message::MoveCusorLeft => {
            return Some(Action::MoveCusorLeft);
        }
        Message::MoveCusorRight => {
            return Some(Action::MoveCusorRight);
        }
        Message::Submit => {
            if let Some(id) = state.workspace_id() {
                return Some(Action::GoToUpdateWorkspace(id));
            } else {
                return Some(Action::GoToCreateWorkspace);
            }
        }
        Message::Tab => {
            return Some(Action::SelectNextInput);
        }
        Message::ExecuteCommand | Message::SelectNext | Message::SelectPrevious => {}
    }

    None
}
