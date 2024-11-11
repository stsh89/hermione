use crate::{
    forms::{NotionBackupCredentialsForm, NotionBackupCredentialsFormParameters},
    layouts::WideLayout,
    screen::Popup,
    themes::{Theme, Themed},
    widgets::{Notice, StatusBar, StatusBarWidget},
    BackupCredentialsRoute, Message, NotionBackupCredentialsPresenter, Result, Route,
    SaveNotionBackupCredentialsParams,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{layout::Rect, widgets::Clear, Frame};

pub struct ManageNotionBackupCredentialsModel {
    error_message: Option<String>,
    form: NotionBackupCredentialsForm,
    redirect: Option<Route>,
    status_bar: StatusBar,
    theme: Theme,
}

pub struct ManageNotionBackupCredentialsModelParameters {
    pub credentials: Option<NotionBackupCredentialsPresenter>,
    pub error_message: Option<String>,
    pub theme: Theme,
}

impl Model for ManageNotionBackupCredentialsModel {
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

        self.redner_form(frame, main_area);
        self.redner_status_bar(frame, status_bar_area);

        if let Some(message) = self.error_message.as_ref() {
            self.render_error_message(frame, main_area, message);
        }
    }
}

impl ManageNotionBackupCredentialsModel {
    fn cancel(&mut self) {
        if self.error_message.is_some() {
            self.error_message = None;

            return;
        }

        self.redirect = Some(Route::BackupCredentials(BackupCredentialsRoute::List));
    }

    pub fn new(params: ManageNotionBackupCredentialsModelParameters) -> Self {
        let ManageNotionBackupCredentialsModelParameters {
            credentials,
            theme,
            error_message,
        } = params;

        let status_bar = StatusBar::builder()
            .operation("Manage Notion backup credentials")
            .build();

        Self {
            form: NotionBackupCredentialsForm::new(NotionBackupCredentialsFormParameters {
                theme,
                notion_backup_credentials_presenter: credentials,
            }),
            redirect: None,
            status_bar,
            theme,
            error_message,
        }
    }

    fn toggle_focus(&mut self) {
        self.form.select_next_input();
    }

    fn enter_char(&mut self, c: char) {
        self.form.enter_char(c);
    }

    fn delete_char(&mut self) {
        self.form.delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.form.delete_all_chars();
    }

    fn move_cursor_left(&mut self) {
        self.form.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.form.move_cursor_right();
    }

    fn render_error_message(&self, frame: &mut Frame, area: Rect, message: &str) {
        let notice = Notice::error(message)
            .set_background_color(self.theme.popup_background_color)
            .set_border_style(self.theme.danger_color);

        let popup_area = Popup::new(area).wide_area();

        frame.render_widget(Clear, popup_area);
        frame.render_widget(notice, popup_area);
    }

    fn redner_form(&mut self, frame: &mut Frame, area: Rect) {
        self.form.render(frame, area);
    }

    fn redner_status_bar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            StatusBarWidget::new(&self.status_bar).themed(self.theme),
            area,
        );
    }

    fn submit(&mut self) {
        let NotionBackupCredentialsPresenter {
            api_key,
            commands_database_id,
            workspaces_database_id,
        } = self.form.credentials();

        self.redirect = Some(
            SaveNotionBackupCredentialsParams {
                api_key,
                commands_database_id,
                workspaces_database_id,
            }
            .into(),
        );
    }
}
