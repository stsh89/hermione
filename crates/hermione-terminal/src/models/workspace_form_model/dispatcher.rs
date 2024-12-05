use super::{state::ModelState, Message};
use crate::coordinator::WorkspaceId;

pub enum Action {
    ClearInput,
    DeleteChar,
    EnterChar(char),
    EnterInputMode,
    ExitInputMode,
    GoToWorkspacesList,
    MoveCusorLeft,
    MoveCusorRight,
    GoToCreateWorkspace,
    GoToUpdateWorkspace(WorkspaceId),
    SelectNextInput,
    Stop,
}

pub fn dispatch(action: Action, state: &mut ModelState) {
    match action {
        Action::ClearInput => state.clear_input(),
        Action::DeleteChar => state.delete_char(),
        Action::EnterChar(c) => state.enter_char(c),
        Action::EnterInputMode => state.enter_input_mode(),
        Action::ExitInputMode => state.exit_input_mode(),
        Action::GoToWorkspacesList => state.set_redirect_to_list_workspaces(),
        Action::MoveCusorLeft => state.move_cursor_left(),
        Action::MoveCusorRight => state.move_cursor_right(),
        Action::SelectNextInput => state.select_next_input(),
        Action::GoToCreateWorkspace => state.set_redirect_to_create_workspace(),
        Action::GoToUpdateWorkspace(id) => state.set_redirect_to_update_workspace(id),
        Action::Stop => state.stop(),
    }
}

pub fn maybe_create_action(message: Message, state: &ModelState) -> Option<Action> {
    match message {
        Message::Cancel => {
            if state.is_in_normal_mode() {
                return Some(Action::GoToWorkspacesList);
            }

            if state.is_in_input_mode() {
                return Some(Action::ExitInputMode);
            }
        }
        Message::DeleteAllChars => {
            if state.is_in_input_mode() {
                return Some(Action::ClearInput);
            }
        }
        Message::DeleteChar => {
            if state.is_in_input_mode() {
                return Some(Action::DeleteChar);
            }
        }
        Message::EnterChar(c) => {
            if state.is_in_input_mode() {
                return Some(Action::EnterChar(c));
            }

            if state.is_in_normal_mode() {
                if c == 'i' {
                    return Some(Action::EnterInputMode);
                }

                if c == 'q' {
                    return Some(Action::Stop);
                }
            }
        }
        Message::MoveCusorLeft => {
            if state.is_in_input_mode() {
                return Some(Action::MoveCusorLeft);
            }
        }
        Message::MoveCusorRight => {
            if state.is_in_input_mode() {
                return Some(Action::MoveCusorRight);
            }
        }
        Message::Submit => {
            if let Some(id) = state.workspace_id() {
                return Some(Action::GoToUpdateWorkspace(id));
            } else {
                return Some(Action::GoToCreateWorkspace);
            }
        }
        Message::Tab => {
            if state.is_in_input_mode() {
                return Some(Action::SelectNextInput);
            }
        }
        Message::ExecuteCommand | Message::SelectNext | Message::SelectPrevious => {}
    }

    None
}
