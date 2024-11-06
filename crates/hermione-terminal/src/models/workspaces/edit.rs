use crate::{
    forms::{EditWorkspaceFormParameters, WorkspaceForm},
    layouts::WideLayout,
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    ListWorkspacesParams, Message, Result, Route, UpdateWorkspaceParams, WorkspacePresenter,
    LIST_WORKSPACES_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::Frame;
use std::num::NonZeroU32;

pub struct EditWorkspaceModel {
    form: WorkspaceForm,
    status_bar: StatusBar,
    redirect: Option<Route>,
    theme: Theme,
}

pub struct EditWorkspaceModelParameters {
    pub workspace: WorkspacePresenter,
    pub theme: Theme,
}

impl Model for EditWorkspaceModel {
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
            Message::Tab => self.toggle_focus(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
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

impl EditWorkspaceModel {
    fn back(&mut self) {
        let WorkspacePresenter {
            id: _,
            name: search_query,
            location: _,
        } = self.form.workspace();

        self.redirect = Some(
            ListWorkspacesParams {
                search_query,
                page_number: NonZeroU32::new(1),
                page_size: NonZeroU32::new(LIST_WORKSPACES_PAGE_SIZE),
            }
            .into(),
        );
    }

    fn delete_char(&mut self) {
        self.form.delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.form.delete_all_chars();
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

    pub fn new(parameters: EditWorkspaceModelParameters) -> Result<Self> {
        let EditWorkspaceModelParameters { workspace, theme } = parameters;

        let status_bar = StatusBar::builder()
            .operation("Edit workspace")
            .workspace(&workspace.name)
            .build();

        Ok(Self {
            redirect: None,
            status_bar,
            form: WorkspaceForm::edit(EditWorkspaceFormParameters { workspace, theme }),
            theme,
        })
    }

    fn submit(&mut self) {
        let WorkspacePresenter { id, name, location } = self.form.workspace();

        self.redirect = Some(UpdateWorkspaceParams { name, location, id }.into());
    }

    fn toggle_focus(&mut self) {
        self.form.select_next_input();
    }
}
