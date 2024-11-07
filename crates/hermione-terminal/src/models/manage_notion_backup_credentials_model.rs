use crate::{
    forms::{NotionBackupCredentialsForm, NotionBackupCredentialsFormParameters},
    layouts::WideLayout,
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    BackupCredentialsRoute, Message, NotionBackupCredentialsPresenter, Result, Route,
    SaveNotionBackupCredentialsParams,
};
use hermione_tui::{EventHandler, Model};
use ratatui::Frame;

pub struct ManageNotionBackupCredentialsModel {
    status_bar: StatusBar,
    redirect: Option<Route>,
    form: NotionBackupCredentialsForm,
    theme: Theme,
}

pub struct ManageNotionBackupCredentialsModelParameters {
    pub theme: Theme,
    pub credentials: Option<NotionBackupCredentialsPresenter>,
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
            Message::Cancel => self.back(),
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

        self.form.render(frame, main_area);

        frame.render_widget(
            StatusBarWidget::new(&self.status_bar).themed(self.theme),
            status_bar_area,
        );
    }
}

impl ManageNotionBackupCredentialsModel {
    fn back(&mut self) {
        self.redirect = Some(Route::BackupCredentials(BackupCredentialsRoute::List));
    }

    pub fn new(params: ManageNotionBackupCredentialsModelParameters) -> Self {
        let ManageNotionBackupCredentialsModelParameters { credentials, theme } = params;

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
