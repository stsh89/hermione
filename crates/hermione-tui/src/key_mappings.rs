use crate::{
    models::{
        change_location::Message as ChangeLocationMessage,
        command_center::Message as CommandCenterMessage,
        command_display::Message as CommandDisplayMessage, lobby::Message as LobbyMessage,
        new_command::Message as NewCommandMessage,
    },
    Result,
};
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

pub enum InputMode {
    Normal,
    Editing,
}

impl InputMode {
    pub fn is_editing(&self) -> bool {
        matches!(self, InputMode::Editing)
    }
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

pub fn command_center_key_mapping(
    key_event: KeyEvent,
    mode: InputMode,
) -> Result<Option<CommandCenterMessage>> {
    use CommandCenterMessage as CCM;

    let message = match key_event.code {
        KeyCode::Char(c) if mode.is_editing() => Some(CCM::EnterChar(c)),
        KeyCode::Left if mode.is_editing() => Some(CCM::MoveCusorLeft),
        KeyCode::Right if mode.is_editing() => Some(CCM::MoveCusorRight),
        KeyCode::Backspace if mode.is_editing() => Some(CCM::DeleteChar),
        KeyCode::Up => Some(CCM::SelectPreviousCommand),
        KeyCode::Down => Some(CCM::SelectNextCommand),
        KeyCode::Esc => Some(CCM::Exit),
        KeyCode::Char('n') => Some(CCM::NewCommandRequest),
        KeyCode::Char('d') => Some(CCM::DeleteCommand),
        KeyCode::Char('c') => Some(CCM::ChangeLocationRequest),
        KeyCode::Enter if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
            Some(CCM::RunCommand)
        }
        KeyCode::Enter => Some(CCM::ExecuteCommand),
        KeyCode::Char('s') => Some(CCM::ActivateSearchBar),
        _ => None,
    };

    Ok(message)
}
