use super::{state::ModelState, Message};
use crate::coordinator::WorkspaceId;

pub enum Action {
    GoToEditWokspace(WorkspaceId),
    GoToNewWorkspaceForm,
    GoToWorkspaceCommands(WorkspaceId),
    LoadNextPage,
    LoadPreviousPage,
    Search(SearchAction),
    SelectNextWorkspace,
    SelectPreviousWorkspace,
    Stop,
}

pub enum SearchAction {
    Clear,
    DeleteChar,
    Discard,
    End,
    EnterChar(char),
    MoveCursorLeft,
    MoveCursorRight,
    Start,
}

pub fn dispatch(action: Action, state: &mut ModelState) {
    match action {
        Action::Search(action) => match action {
            SearchAction::Clear => state.clear_search_query(),
            SearchAction::DeleteChar => state.edit_search_query(),
            SearchAction::Discard => state.discard_search_query(),
            SearchAction::End => state.enter_normal_mode(),
            SearchAction::EnterChar(c) => state.update_search_query(c),
            SearchAction::MoveCursorLeft => state.move_search_query_cursor_left(),
            SearchAction::MoveCursorRight => state.move_search_query_cursor_right(),
            SearchAction::Start => state.enter_search_mode(),
        },
        Action::GoToEditWokspace(id) => state.set_workspace_redirect(id),
        Action::GoToNewWorkspaceForm => state.set_new_workspace_redirect(),
        Action::GoToWorkspaceCommands(id) => state.set_commands_redirect(id),
        Action::LoadNextPage => state.set_next_page_redirect(),
        Action::LoadPreviousPage => state.set_previous_page_redirect(),
        Action::SelectNextWorkspace => state.select_next_workspace(),
        Action::SelectPreviousWorkspace => state.select_previous_workspace(),
        Action::Stop => state.stop(),
    }
}

pub fn maybe_create_action(message: Message, state: &ModelState) -> Option<Action> {
    match message {
        Message::Cancel => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::Discard));
            }
        }
        Message::DeleteAllChars => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::Clear));
            }
        }
        Message::DeleteChar => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::DeleteChar));
            }
        }
        Message::EnterChar(c) => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::EnterChar(c)));
            }

            if state.is_in_normal_mode() {
                if c == '/' {
                    return Some(Action::Search(SearchAction::Start));
                }

                if c == 'c' && state.is_in_normal_mode() {
                    if let Some(id) = state.selected_workspace_id() {
                        return Some(Action::GoToWorkspaceCommands(id));
                    }
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

                if c == 'j' {
                    return Some(select_next(state));
                }

                if c == 'k' {
                    return Some(select_previous(state));
                }
            }
        }
        Message::MoveCusorLeft => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::MoveCursorLeft));
            }
        }
        Message::MoveCusorRight => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::MoveCursorRight));
            }
        }
        Message::SelectNext => return Some(select_next(state)),
        Message::SelectPrevious => return Some(select_previous(state)),
        Message::Submit => {
            if state.is_in_search_mode() {
                return Some(Action::Search(SearchAction::End));
            }

            if state.is_in_normal_mode() {
                if let Some(id) = state.selected_workspace_id() {
                    return Some(Action::GoToEditWokspace(id));
                }
            }
        }
        Message::ExecuteCommand | Message::Tab => {}
    };

    None
}

fn select_next(state: &ModelState) -> Action {
    if state.is_in_search_mode() {
        return Action::SelectNextWorkspace;
    }

    if state.is_last_workspace_selected() {
        Action::LoadNextPage
    } else {
        Action::SelectNextWorkspace
    }
}

fn select_previous(state: &ModelState) -> Action {
    if state.is_in_search_mode() {
        return Action::SelectPreviousWorkspace;
    }

    if state.is_first_workspace_selected() {
        Action::LoadPreviousPage
    } else {
        Action::SelectPreviousWorkspace
    }
}
