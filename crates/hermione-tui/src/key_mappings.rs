use crate::{
    models::{
        command_display::Message as CommandDisplayMessage, lobby::Message as LobbyMessage,
        new_command::Message as NewCommandMessage, new_workspace::Message as NewWorkspaceMessage,
    },
    Result,
};
use ratatui::crossterm::event::KeyCode;

pub fn lobby_key_mapping(key_code: KeyCode) -> Result<Option<LobbyMessage>> {
    use LobbyMessage as LM;

    let message = match key_code {
        KeyCode::Up => Some(LM::SelectPreviousWorkspace),
        KeyCode::Down => Some(LM::SelectNextWorkspace),
        KeyCode::Char('n') => Some(LM::NewWorkspaceRequest),
        KeyCode::Char('d') => Some(LM::DeleteWorkspace),
        KeyCode::Esc => Some(LM::Exit),
        KeyCode::Enter => Some(LM::EnterCommandCenter),
        KeyCode::Char('q') => Some(LM::Exit),
        _ => None,
    };

    Ok(message)
}

pub fn new_workspace_key_mapping(key_code: KeyCode) -> Result<Option<NewWorkspaceMessage>> {
    use NewWorkspaceMessage as NWM;

    let message = match key_code {
        KeyCode::Left => Some(NWM::MoveCusorLeft),
        KeyCode::Right => Some(NWM::MoveCusorRight),
        KeyCode::Char(c) => Some(NWM::EnterChar(c)),
        KeyCode::Backspace => Some(NWM::DeleteChar),
        KeyCode::Enter => Some(NWM::Submit),
        KeyCode::Esc => Some(NWM::Exit),
        _ => None,
    };

    Ok(message)
}

pub fn new_command_key_mapping(key_code: KeyCode) -> Result<Option<NewCommandMessage>> {
    use NewCommandMessage as NCM;

    let message = match key_code {
        KeyCode::Left => Some(NCM::MoveCusorLeft),
        KeyCode::Right => Some(NCM::MoveCusorRight),
        KeyCode::Char(c) => Some(NCM::EnterChar(c)),
        KeyCode::Backspace => Some(NCM::DeleteChar),
        KeyCode::Enter => Some(NCM::Submit),
        KeyCode::Esc => Some(NCM::Exit),
        KeyCode::Tab => Some(NCM::ToggleFormInput),
        _ => None,
    };

    Ok(message)
}

pub fn command_display_key_mapping(key_code: KeyCode) -> Result<Option<CommandDisplayMessage>> {
    use CommandDisplayMessage as CDM;

    let message = match key_code {
        KeyCode::Esc => Some(CDM::Exit),
        KeyCode::Char('r') => Some(CDM::RepeatCommand),
        _ => None,
    };

    Ok(message)
}
