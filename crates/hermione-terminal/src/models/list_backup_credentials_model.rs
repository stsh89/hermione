use crate::{
    layouts::{SearchListLayout, WideLayout},
    models::BackupProviderKind,
    smart_input::{NewSmartInputParameters, SmartInput},
    themes::{Theme, HIGHLIGHT_SYMBOL},
    BackupCredentialsRoute, DeleteBackupCredentialsParams, Error, ExportParams, ImportParams,
    ListWorkspacesParams, Message, Result, Route,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{
    layout::Rect,
    style::Stylize,
    text::ToText,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};

pub struct ListBackupCredentialsModel {
    is_running: bool,
    redirect: Option<Route>,
    smart_input: SmartInput,
    backup_credentials_kinds: Vec<BackupProviderKind>,
    backup_credentials_kinds_state: ListState,
    theme: Theme,
}

pub struct ListBackupCredentialsModelParameters {
    pub backup_credentials_kinds: Vec<BackupProviderKind>,
    pub theme: Theme,
}

impl Model for ListBackupCredentialsModel {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Self::Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Self::Message) -> Result<Option<Self::Message>> {
        match message {
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Submit => self.submit()?,
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Tab => self.autocomplete(),
            Message::Cancel => self.cancel(),
            Message::ExecuteCommand => {}
        };

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let [list_area, input_area] = SearchListLayout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());

        let list = List::new(&self.backup_credentials_kinds)
            .block(block)
            .highlight_symbol(HIGHLIGHT_SYMBOL)
            .bg(self.theme.background_color)
            .fg(self.theme.foreground_color)
            .highlight_style(self.theme.highlight_color);

        frame.render_stateful_widget(list, list_area, &mut self.backup_credentials_kinds_state);
        self.smart_input.render(frame, input_area);

        self.render_status_bar(frame, status_bar_area);
    }
}

impl ListBackupCredentialsModel {
    fn autocomplete(&mut self) {
        self.smart_input.autocomplete();
    }

    fn backup_credentials_kind(&self) -> Option<&BackupProviderKind> {
        self.backup_credentials_kinds_state
            .selected()
            .and_then(|index| self.backup_credentials_kinds.get(index))
    }

    fn cancel(&mut self) {
        self.smart_input.reset_input();
    }

    fn delete_all_chars(&mut self) {
        self.smart_input.reset_input();
    }

    fn delete_char(&mut self) {
        self.smart_input.delete_char();
    }

    fn enter_char(&mut self, c: char) {
        self.smart_input.enter_char(c);
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    fn move_cursor_left(&mut self) {
        self.smart_input.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.smart_input.move_cursor_right();
    }

    pub fn new(parameters: ListBackupCredentialsModelParameters) -> Self {
        let ListBackupCredentialsModelParameters {
            backup_credentials_kinds,
            theme,
        } = parameters;

        let mut backup_credentials_kinds_state = ListState::default();

        if !backup_credentials_kinds.is_empty() {
            backup_credentials_kinds_state.select_first();
        }

        let smart_input = SmartInput::new(NewSmartInputParameters {
            theme,
            commands: Action::all().into_iter().map(Into::into).collect(),
        });

        ListBackupCredentialsModel {
            theme,
            is_running: true,
            backup_credentials_kinds,
            backup_credentials_kinds_state,
            smart_input,
            redirect: None,
        }
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let value = serde_json::json!({
            "screen": "Backup credentials",
        });

        let paragraph = Paragraph::new(value.to_text())
            .bg(self.theme.status_bar_background_color)
            .fg(self.theme.status_bar_foreground_color);

        frame.render_widget(paragraph, area);
    }

    fn select_next(&mut self) {
        self.backup_credentials_kinds_state.select_next();
    }

    fn select_previous(&mut self) {
        self.backup_credentials_kinds_state.select_previous();
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn submit(&mut self) -> Result<()> {
        let Some(command) = self.smart_input.command() else {
            self.smart_input.reset_input();
            return Ok(());
        };

        let action = Action::try_from(command)?;

        match action {
            Action::DeleteBackupCredentials => {
                if let Some(kind) = self.backup_credentials_kind().cloned() {
                    self.set_redirect(DeleteBackupCredentialsParams { kind }.into());
                }
            }
            Action::EditBackupCredentials => {}
            Action::Exit => self.exit(),
            Action::Export => {
                if let Some(kind) = self.backup_credentials_kind().cloned() {
                    self.smart_input.reset_input();
                    self.set_redirect(ExportParams { kind }.into());
                }
            }

            Action::Import => {
                if let Some(kind) = self.backup_credentials_kind().cloned() {
                    self.smart_input.reset_input();
                    self.set_redirect(ImportParams { kind }.into());
                }
            }
            Action::ListWorkspaces => {
                self.set_redirect(ListWorkspacesParams::default().into());
            }
            Action::ManageNotionBackupCredentials => {
                self.set_redirect(Route::BackupCredentials(
                    BackupCredentialsRoute::ManageNotionBackupCredentials,
                ));
            }
        }

        Ok(())
    }
}

enum Action {
    DeleteBackupCredentials,
    EditBackupCredentials,
    Exit,
    Import,
    Export,
    ManageNotionBackupCredentials,
    ListWorkspaces,
}

impl Action {
    fn all() -> Vec<Self> {
        vec![
            Self::DeleteBackupCredentials,
            Self::Exit,
            Self::ManageNotionBackupCredentials,
            Self::EditBackupCredentials,
            Self::ListWorkspaces,
            Self::Import,
            Self::Export,
        ]
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        let action = match action {
            Action::DeleteBackupCredentials => "Delete backup credentials",
            Action::EditBackupCredentials => "Edit backup credentials",
            Action::Exit => "Exit",
            Action::ManageNotionBackupCredentials => "Manage Notion backup credentials",
            Action::ListWorkspaces => "List workspaces",
            Action::Import => "Import",
            Action::Export => "Export",
        };

        action.to_string()
    }
}

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let action = match value {
            "Delete backup credentials" => Self::DeleteBackupCredentials,
            "Exit" => Self::Exit,
            "Edit backup credentials" => Self::EditBackupCredentials,
            "Manage Notion backup credentials" => Self::ManageNotionBackupCredentials,
            "List workspaces" => Self::ListWorkspaces,
            "Import" => Self::Import,
            "Export" => Self::Export,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unknown backup credentials action: {}",
                    value
                ))
            }
        };

        Ok(action)
    }
}
