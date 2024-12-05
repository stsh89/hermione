use super::{state::ModelState, Message};
use crate::coordinator::WorkspaceId;

pub enum Action {
    ClearInput,
    DeleteChar,
    EnterChar(char),
    EnterInputMode,
    ExitInputMode,
    ListWorkspaces,
    MoveCusorLeft,
    MoveCusorRight,
    SetRedirectToCreateWorkspace,
    SetRedirectToUpdateWorkspace(WorkspaceId),
    SelectNextInput,
    Stop,
}

pub fn dispatch(action: Action, screen: &mut ModelState) {
    match action {
        Action::ClearInput => screen.clear_input(),
        Action::DeleteChar => screen.delete_char(),
        Action::EnterChar(c) => screen.enter_char(c),
        Action::EnterInputMode => screen.enter_input_mode(),
        Action::ExitInputMode => screen.exit_input_mode(),
        Action::ListWorkspaces => screen.list_workspaces(),
        Action::MoveCusorLeft => screen.move_cursor_left(),
        Action::MoveCusorRight => screen.move_cursor_right(),
        Action::SelectNextInput => screen.select_next_input(),
        Action::SetRedirectToCreateWorkspace => screen.set_redirect_to_create_workspace(),
        Action::SetRedirectToUpdateWorkspace(workspace_id) => {
            screen.set_redirect_to_update_workspace(workspace_id)
        }
        Action::Stop => screen.stop(),
    }
}

pub fn maybe_create_action(message: Message, screen: &ModelState) -> Option<Action> {
    match message {
        Message::Cancel => {
            if screen.is_in_normal_mode() {
                return Some(Action::ListWorkspaces);
            }

            if screen.is_in_input_mode() {
                return Some(Action::ExitInputMode);
            }
        }
        Message::DeleteAllChars => {
            if screen.is_in_input_mode() {
                return Some(Action::ClearInput);
            }
        }
        Message::DeleteChar => {
            if screen.is_in_input_mode() {
                return Some(Action::DeleteChar);
            }
        }
        Message::EnterChar(c) => {
            if screen.is_in_input_mode() {
                return Some(Action::EnterChar(c));
            }

            if screen.is_in_normal_mode() {
                if c == 'i' {
                    return Some(Action::EnterInputMode);
                }

                if c == 'q' {
                    return Some(Action::Stop);
                }
            }
        }
        Message::MoveCusorLeft => {
            if screen.is_in_input_mode() {
                return Some(Action::MoveCusorLeft);
            }
        }
        Message::MoveCusorRight => {
            if screen.is_in_input_mode() {
                return Some(Action::MoveCusorRight);
            }
        }
        Message::Submit => {
            if let Some(id) = screen.workspace_id() {
                return Some(Action::SetRedirectToUpdateWorkspace(id));
            } else {
                return Some(Action::SetRedirectToCreateWorkspace);
            }
        }
        Message::Tab => {
            if screen.is_in_input_mode() {
                return Some(Action::SelectNextInput);
            }
        }
        Message::ExecuteCommand | Message::SelectNext | Message::SelectPrevious => {}
    }

    None
}
