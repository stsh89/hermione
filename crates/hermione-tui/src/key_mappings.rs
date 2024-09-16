use crate::{
    models::{
        command_center::Message as CommandCenterMessage,
        command_display::Message as CommandDisplayMessage, lobby::Message as LobbyMessage,
        new_command::Message as NewCommandMessage, new_workspace::Message as NewWorkspaceMessage,
    },
    Result,
};
use ratatui::crossterm::event::KeyCode;

pub enum InputMode {
    Normal,
    Editing,
}

impl InputMode {
    pub fn is_editing(&self) -> bool {
        matches!(self, InputMode::Editing)
    }
}

pub fn lobby_key_mapping(code: KeyCode, _mode: InputMode) -> Result<Option<LobbyMessage>> {
    use LobbyMessage as LM;

    let message = match code {
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

pub fn new_workspace_key_mapping(
    code: KeyCode,
    _mode: InputMode,
) -> Result<Option<NewWorkspaceMessage>> {
    use NewWorkspaceMessage as NWM;

    let message = match code {
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

pub fn new_command_key_mapping(
    code: KeyCode,
    _mode: InputMode,
) -> Result<Option<NewCommandMessage>> {
    use NewCommandMessage as NCM;

    let message = match code {
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
    code: KeyCode,
    _mode: InputMode,
) -> Result<Option<CommandDisplayMessage>> {
    use CommandDisplayMessage as CDM;

    let message = match code {
        KeyCode::Esc => Some(CDM::Exit),
        KeyCode::Char('r') => Some(CDM::RepeatCommand),
        _ => None,
    };

    Ok(message)
}

pub fn command_center_key_mapping(
    code: KeyCode,
    mode: InputMode,
) -> Result<Option<CommandCenterMessage>> {
    use CommandCenterMessage as CCM;

    let message = match code {
        KeyCode::Char(c) if mode.is_editing() => Some(CCM::EnterChar(c)),
        KeyCode::Left if mode.is_editing() => Some(CCM::MoveCusorLeft),
        KeyCode::Right if mode.is_editing() => Some(CCM::MoveCusorRight),
        KeyCode::Backspace if mode.is_editing() => Some(CCM::DeleteChar),
        KeyCode::Up => Some(CCM::SelectPreviousCommand),
        KeyCode::Down => Some(CCM::SelectNextCommand),
        KeyCode::Esc => Some(CCM::Exit),
        KeyCode::Char('n') => Some(CCM::NewCommandRequest),
        KeyCode::Char('d') => Some(CCM::DeleteCommand),
        KeyCode::Enter => Some(CCM::ExecuteCommand),
        KeyCode::Char('s') => Some(CCM::ActivateSearchBar),
        _ => None,
    };

    Ok(message)
}