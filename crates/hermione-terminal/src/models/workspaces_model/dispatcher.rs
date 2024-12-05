use super::{state::ModelState, Message};
use crate::coordinator::WorkspaceId;

pub enum Action {
    EnterSearchMode,
    SelectNextWorkspace,
    SelectPreviousWorkspace,
    LoadNextPage,
    LoadPreviousPage,
    ExitSearchMode,
    UpdateSearchQuery(char),
    DeleteSearchQuery,
    MoveSearchQueryCusorLeft,
    MoveSearchQueryCusorRight,
    EditSearchQuery,
    GoToNewWorkspaceForm,
    GoToEditWokspace(WorkspaceId),
    Stop,
    GoToWorkspaceCommands(WorkspaceId),
}

pub fn dispatch(action: Action, state: &mut ModelState) {
    match action {
        Action::DeleteSearchQuery => state.delete_search_query(),
        Action::EditSearchQuery => state.edit_search_query(),
        Action::EnterSearchMode => state.enter_search_mode(),
        Action::ExitSearchMode => state.exit_search_mode(),
        Action::GoToEditWokspace(id) => state.set_workspace_redirect(id),
        Action::GoToNewWorkspaceForm => state.set_new_workspace_redirect(),
        Action::GoToWorkspaceCommands(id) => state.set_commands_redirect(id),
        Action::LoadNextPage => state.set_next_page_redirect(),
        Action::LoadPreviousPage => state.set_previous_page_redirect(),
        Action::MoveSearchQueryCusorLeft => state.move_search_query_cursor_left(),
        Action::MoveSearchQueryCusorRight => state.move_search_query_cursor_right(),
        Action::SelectNextWorkspace => state.select_next_workspace(),
        Action::SelectPreviousWorkspace => state.select_previous_workspace(),
        Action::Stop => state.stop(),
        Action::UpdateSearchQuery(c) => state.update_search_query(c),
    }
}

pub fn maybe_create_action(message: Message, state: &ModelState) -> Option<Action> {
    match message {
        Message::Cancel => {
            if state.is_in_search_mode() {
                return Some(Action::ExitSearchMode);
            }
        }
        Message::DeleteAllChars => {
            if state.is_in_search_mode() {
                return Some(Action::DeleteSearchQuery);
            }
        }
        Message::DeleteChar => {
            if state.is_in_search_mode() {
                return Some(Action::EditSearchQuery);
            }
        }
        Message::EnterChar(c) => {
            if state.is_in_normal_mode() {
                if c == '/' {
                    return Some(Action::EnterSearchMode);
                }

                if c == 'n' {
                    return Some(Action::GoToNewWorkspaceForm);
                }

                if c == 'q' {
                    return Some(Action::Stop);
                }

                if c == 'e' {
                    if let Some(id) = state.selected_workspace_id() {
                        return Some(Action::GoToEditWokspace(id));
                    }
                }
            }

            if state.is_in_search_mode() {
                return Some(Action::UpdateSearchQuery(c));
            }
        }
        Message::MoveCusorLeft => {
            if state.is_in_search_mode() {
                return Some(Action::MoveSearchQueryCusorLeft);
            }
        }
        Message::MoveCusorRight => {
            if state.is_in_search_mode() {
                return Some(Action::MoveSearchQueryCusorRight);
            }
        }
        Message::SelectNext => {
            if state.is_in_search_mode() {
                return Some(Action::SelectNextWorkspace);
            }

            if state.is_last_workspace_selected() {
                return Some(Action::LoadNextPage);
            } else {
                return Some(Action::SelectNextWorkspace);
            }
        }
        Message::SelectPrevious => {
            if state.is_in_search_mode() {
                return Some(Action::SelectPreviousWorkspace);
            }

            if state.is_first_workspace_selected() {
                return Some(Action::LoadPreviousPage);
            } else {
                return Some(Action::SelectPreviousWorkspace);
            }
        }
        Message::Submit => {
            if let Some(id) = state.selected_workspace_id() {
                return Some(Action::GoToWorkspaceCommands(id));
            }
        }
        Message::ExecuteCommand | Message::Tab => {}
    };

    None
}
