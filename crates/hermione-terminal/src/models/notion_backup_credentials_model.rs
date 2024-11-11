use crate::{
    layouts::WideLayout,
    screen::Popup,
    themes::{Theme, Themed},
    widgets::{FormField, Notice, StatusBar, StatusBarWidget},
    BackupCredentialsRoute, Message, Result, Route, SaveNotionBackupCredentialsParams,
};
use hermione_tui::{EventHandler, Input, Model};
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    widgets::Clear,
    Frame,
};

pub struct NotionBackupCredentialsModel {
    active_input: InputName,
    api_key: Input,
    commands_database_id: Input,
    error_message: Option<String>,
    redirect: Option<Route>,
    status_bar: StatusBar,
    theme: Theme,
    workspaces_database_id: Input,
}

#[derive(Default, PartialEq)]
enum InputName {
    #[default]
    ApiKey,
    CommandsDatabaseId,
    WorkspacesDatabaseId,
}

impl Model for NotionBackupCredentialsModel {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Cancel => self.cancel(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            Message::Tab => self.toggle_focus(),
            Message::ExecuteCommand | Message::SelectNext | Message::SelectPrevious => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let input_areas = input_areas(main_area);

        self.render_api_key(frame, input_areas[0]);
        self.render_commands_database_id(frame, input_areas[1]);
        self.render_workspaces_database_id(frame, input_areas[2]);
        self.render_status_bar(frame, status_bar_area);

        self.set_cursor_position(frame, input_areas[self.active_input.as_index()]);

        if let Some(message) = self.error_message.as_ref() {
            self.render_error_message(frame, main_area, message);
        }
    }
}

impl NotionBackupCredentialsModel {
    fn active_input(&self) -> &Input {
        match self.active_input {
            InputName::ApiKey => &self.api_key,
            InputName::CommandsDatabaseId => &self.commands_database_id,
            InputName::WorkspacesDatabaseId => &self.workspaces_database_id,
        }
    }

    pub fn api_key(self, api_key: String) -> Self {
        Self {
            api_key: Input::new(api_key),
            ..self
        }
    }

    fn cancel(&mut self) {
        if self.error_message.is_some() {
            self.error_message = None;

            return;
        }

        self.redirect = Some(Route::BackupCredentials(BackupCredentialsRoute::List));
    }

    pub fn commands_database_id(self, commands_database_id: String) -> Self {
        Self {
            commands_database_id: Input::new(commands_database_id),
            ..self
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            InputName::ApiKey => self.api_key.delete_all_chars(),
            InputName::CommandsDatabaseId => self.commands_database_id.delete_all_chars(),
            InputName::WorkspacesDatabaseId => self.workspaces_database_id.delete_all_chars(),
        }
    }

    fn delete_char(&mut self) {
        match self.active_input {
            InputName::ApiKey => self.api_key.delete_char(),
            InputName::CommandsDatabaseId => self.commands_database_id.delete_char(),
            InputName::WorkspacesDatabaseId => self.workspaces_database_id.delete_char(),
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            InputName::ApiKey => self.api_key.enter_char(c),
            InputName::CommandsDatabaseId => self.commands_database_id.enter_char(c),
            InputName::WorkspacesDatabaseId => self.workspaces_database_id.enter_char(c),
        }
    }

    pub fn error_message(self, error_message: String) -> Self {
        Self {
            error_message: Some(error_message),
            ..self
        }
    }

    pub fn theme(self, theme: Theme) -> Self {
        Self { theme, ..self }
    }

    fn toggle_focus(&mut self) {
        self.active_input = self.active_input.next();
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            InputName::ApiKey => self.api_key.move_cursor_left(),
            InputName::CommandsDatabaseId => self.commands_database_id.move_cursor_left(),
            InputName::WorkspacesDatabaseId => self.workspaces_database_id.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            InputName::ApiKey => self.api_key.move_cursor_right(),
            InputName::CommandsDatabaseId => self.commands_database_id.move_cursor_right(),
            InputName::WorkspacesDatabaseId => self.workspaces_database_id.move_cursor_right(),
        }
    }

    fn render_api_key(&self, frame: &mut Frame, area: Rect) {
        let value = &self.api_key.secret_value();

        let mut field = FormField::default()
            .name(InputName::ApiKey.as_str())
            .value(value)
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::ApiKey == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_commands_database_id(&self, frame: &mut Frame, area: Rect) {
        let value = &self.commands_database_id.secret_value();

        let mut field = FormField::default()
            .name(InputName::CommandsDatabaseId.as_str())
            .value(value)
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::CommandsDatabaseId == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_error_message(&self, frame: &mut Frame, area: Rect, message: &str) {
        let notice = Notice::error(message)
            .set_background_color(self.theme.popup_background_color)
            .set_foreground_color(self.theme.foreground_color)
            .set_border_style(self.theme.danger_color);

        let popup_area = Popup::new(area).wide_area();

        frame.render_widget(Clear, popup_area);
        frame.render_widget(notice, popup_area);
    }

    fn render_workspaces_database_id(&self, frame: &mut Frame, area: Rect) {
        let value = &self.workspaces_database_id.secret_value();
        let mut field = FormField::default()
            .name(InputName::WorkspacesDatabaseId.as_str())
            .value(value)
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::WorkspacesDatabaseId == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_status_bar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            StatusBarWidget::new(&self.status_bar).themed(self.theme),
            area,
        );
    }

    fn set_cursor_position(&self, frame: &mut Frame, area: Rect) {
        frame.set_cursor_position(Position::new(
            area.x + self.active_input().character_index() as u16 + 1,
            area.y + 1,
        ));
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn submit(&mut self) {
        self.set_redirect(
            SaveNotionBackupCredentialsParams {
                api_key: self.api_key.value().to_string(),
                commands_database_id: self.commands_database_id.value().to_string(),
                workspaces_database_id: self.workspaces_database_id.value().to_string(),
            }
            .into(),
        );
    }

    pub fn workspaces_database_id(self, workspaces_database_id: String) -> Self {
        Self {
            workspaces_database_id: Input::new(workspaces_database_id),
            ..self
        }
    }
}

impl InputName {
    fn as_index(&self) -> usize {
        match self {
            InputName::ApiKey => 0,
            InputName::CommandsDatabaseId => 1,
            InputName::WorkspacesDatabaseId => 2,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            InputName::ApiKey => "API key",
            InputName::CommandsDatabaseId => "Commands database ID",
            InputName::WorkspacesDatabaseId => "Workspaces database ID",
        }
    }

    fn next(&self) -> Self {
        match self {
            InputName::ApiKey => InputName::CommandsDatabaseId,
            InputName::CommandsDatabaseId => InputName::WorkspacesDatabaseId,
            InputName::WorkspacesDatabaseId => InputName::ApiKey,
        }
    }
}

fn input_areas(main_area: Rect) -> [Rect; 3] {
    ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Min(3),
        ])
        .areas(main_area)
}

impl Default for NotionBackupCredentialsModel {
    fn default() -> Self {
        let status_bar = StatusBar::builder()
            .operation("Notion backup credentials")
            .build();

        Self {
            active_input: Default::default(),
            api_key: Default::default(),
            commands_database_id: Default::default(),
            error_message: Default::default(),
            redirect: Default::default(),
            status_bar,
            theme: Default::default(),
            workspaces_database_id: Default::default(),
        }
    }
}
