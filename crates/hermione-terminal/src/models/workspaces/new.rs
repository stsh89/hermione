use crate::{
    forms::WorkspaceForm,
    layouts::{self, StatusBar},
    CreateWorkspaceParams, ListWorkspacesParams, Message, Result, Route, WorkspacePresenter,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{widgets::Paragraph, Frame};

pub struct NewWorkspaceModel {
    status_bar: String,
    form: WorkspaceForm,
    redirect: Option<Route>,
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
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());

        self.form.render(frame, main_area);

        let paragraph = Paragraph::new(self.status_bar.as_str());
        frame.render_widget(paragraph, status_bar_area);
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

    pub fn new() -> Result<Self> {
        let status_bar = StatusBar::default().use_case("New workspace").try_into()?;

        Ok(Self {
            status_bar,
            form: WorkspaceForm::default(),
            redirect: None,
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
