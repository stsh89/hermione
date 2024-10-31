use crate::{
    forms::{NewWorkspaceFormParameters, WorkspaceForm},
    layouts::WideLayout,
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    CreateWorkspaceParams, ListWorkspacesParams, Message, Result, Route, WorkspacePresenter,
};
use hermione_tui::{EventHandler, Model};
use ratatui::Frame;

pub struct NewWorkspaceModel {
    status_bar: StatusBar,
    form: WorkspaceForm,
    redirect: Option<Route>,
    theme: Theme,
}

pub struct NewWorkspaceModelParameters {
    pub theme: Theme,
}

impl Model for NewWorkspaceModel {
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

impl NewWorkspaceModel {
    fn back(&mut self) {
        self.redirect = Some(ListWorkspacesParams::default().into());
    }

    fn delete_all_chars(&mut self) {
        self.form.delete_all_chars();
    }

    fn delete_char(&mut self) {
        self.form.delete_char();
    }

    fn enter_char(&mut self, c: char) {
        self.form.enter_char(c);
    }

    fn move_cursor_left(&mut self) {
        self.form.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.form.move_cursor_right();
    }

    pub fn new(parameters: NewWorkspaceModelParameters) -> Result<Self> {
        let NewWorkspaceModelParameters { theme } = parameters;
        let status_bar = StatusBar::builder().operation("New workspace").build();

        Ok(Self {
            status_bar,
            form: WorkspaceForm::new(NewWorkspaceFormParameters { theme }),
            redirect: None,
            theme,
        })
    }

    fn submit(&mut self) {
        let WorkspacePresenter {
            id: _,
            name,
            location,
        } = self.form.workspace();

        self.redirect = Some(CreateWorkspaceParams { name, location }.into());
    }

    fn toggle_focus(&mut self) {
        self.form.select_next_input();
    }
}
