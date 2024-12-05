use super::{screen::Screen, sygnal::Sygnal, Message};

pub fn create_sygnal(message: Message, screen: &Screen) -> Option<Sygnal> {
    match message {
        Message::Cancel => {
            if screen.is_in_normal_mode() {
                return Some(Sygnal::ListWorkspaces);
            }

            if screen.is_in_input_mode() {
                return Some(Sygnal::ExitInputMode);
            }
        }
        Message::DeleteAllChars => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::ClearInput);
            }
        }
        Message::DeleteChar => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::DeleteChar);
            }
        }
        Message::EnterChar(c) => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::EnterChar(c));
            }

            if screen.is_in_normal_mode() {
                if c == 'i' {
                    return Some(Sygnal::EnterInputMode);
                }

                if c == 'q' {
                    return Some(Sygnal::Exit);
                }
            }
        }
        Message::MoveCusorLeft => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::MoveCusorLeft);
            }
        }
        Message::MoveCusorRight => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::MoveCusorRight);
            }
        }
        Message::Submit => {
            if let Some(id) = screen.workspace_id() {
                return Some(Sygnal::UpdateWorkspace(id));
            } else {
                return Some(Sygnal::CreateWorkspace);
            }
        }
        Message::Tab => {
            if screen.is_in_input_mode() {
                return Some(Sygnal::SelectNextInput);
            }
        }
        Message::ExecuteCommand | Message::SelectNext | Message::SelectPrevious => {}
    }

    None
}

pub fn update_screen(sygnal: Sygnal, screen: &mut Screen) {
    match sygnal {
        Sygnal::ClearInput => screen.clear_input(),
        Sygnal::CreateWorkspace => screen.create_workspace(),
        Sygnal::DeleteChar => screen.delete_char(),
        Sygnal::EnterChar(c) => screen.enter_char(c),
        Sygnal::EnterInputMode => screen.enter_input_mode(),
        Sygnal::Exit => screen.exit(),
        Sygnal::ExitInputMode => screen.exit_input_mode(),
        Sygnal::ListWorkspaces => screen.list_workspaces(),
        Sygnal::MoveCusorLeft => screen.move_cursor_left(),
        Sygnal::MoveCusorRight => screen.move_cursor_right(),
        Sygnal::SelectNextInput => screen.select_next_input(),
        Sygnal::UpdateWorkspace(workspace_id) => screen.update_workspace(workspace_id),
    }
}
