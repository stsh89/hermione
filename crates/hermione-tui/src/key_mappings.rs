use crate::{
    models::{
        change_location::Message as ChangeLocationMessage,
        command_display::Message as CommandDisplayMessage, lobby::Message as LobbyMessage,
        new_command::Message as NewCommandMessage,
    },
    Result,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

pub enum InputMode {
    Normal,
    Editing,
}

pub fn lobby_key_mapping(key_event: KeyEvent, _mode: InputMode) -> Result<Option<LobbyMessage>> {
    use LobbyMessage as LM;

    let message = match key_event.code {
        KeyCode::Up => Some(LM::SelectPreviousWorkspace),
        KeyCode::Down => Some(LM::SelectNextWorkspace),
        KeyCode::Char('n') => Some(LM::NewWorkspaceRequest),
        KeyCode::Char('d') => Some(LM::DeleteWorkspace),
        KeyCode::Esc => Some(LM::Exit),
        KeyCode::Enter => Some(LM::EnterCommandCenter),
        KeyCode::Char('q') => Some(LM::Exit),
        KeyCode::Char('e') => Some(LM::RenameWorkspace),
        _ => None,
    };

    Ok(message)
}

pub fn change_location_key_mapping(
    key_event: KeyEvent,
    _mode: InputMode,
) -> Result<Option<ChangeLocationMessage>> {
    use ChangeLocationMessage as CLM;

    let message = match key_event.code {
        KeyCode::Left => Some(CLM::MoveCusorLeft),
        KeyCode::Right => Some(CLM::MoveCusorRight),
        KeyCode::Char(c) => Some(CLM::EnterChar(c)),
        KeyCode::Backspace => Some(CLM::DeleteChar),
        KeyCode::Enter => Some(CLM::Submit),
        KeyCode::Esc => Some(CLM::Exit),
        _ => None,
    };

    Ok(message)
}

pub fn new_command_key_mapping(
    key_event: KeyEvent,
    _mode: InputMode,
) -> Result<Option<NewCommandMessage>> {
    use NewCommandMessage as NCM;

    let message = match key_event.code {
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

pub fn command_display_key_mapping(
    key_event: KeyEvent,
    _mode: InputMode,
) -> Result<Option<CommandDisplayMessage>> {
    use CommandDisplayMessage as CDM;

    let message = match key_event.code {
        KeyCode::Esc => Some(CDM::Exit),
        KeyCode::Char('r') => Some(CDM::RepeatCommand),
        _ => None,
    };

    Ok(message)
}
