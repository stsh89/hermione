use crate::{models::lobby::Message, Result};
use ratatui::crossterm::event::KeyCode;

pub fn lobby_key_mapping<'a>(key_code: KeyCode) -> Result<Option<Message>> {
    let message = match key_code {
        KeyCode::Up => Some(Message::SelectPreviousWorkspace),
        KeyCode::Down => Some(Message::SelectNextWorkspace),
        KeyCode::Char('n') => Some(Message::NewWorkspaceRequest),
        KeyCode::Char('d') => Some(Message::DeleteWorkspace),
        KeyCode::Esc => Some(Message::Exit),
        KeyCode::Enter => Some(Message::EnterCommandCenter),
        KeyCode::Char('q') => Some(Message::Exit),
        _ => None,
    };

    Ok(message)
}
