use crate::{
    forms::{CommandForm, NewCommandFormParameters},
    layouts::WideLayout,
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    CommandPresenter, CreateWorkspaceCommandParams, ListWorkspaceCommandsParams, Message, Result,
    Route, WorkspacePresenter, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::Frame;
use std::num::NonZeroU32;

pub struct NewWorkspaceCommandModel {
    status_bar: StatusBar,
    form: CommandForm,
    redirect: Option<Route>,
    theme: Theme,
}

pub struct NewWorkspaceCommandModelParameters {
    pub workspace: WorkspacePresenter,
    pub theme: Theme,
}

impl Model for NewWorkspaceCommandModel {
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

impl NewWorkspaceCommandModel {
    fn back(&mut self) {
        let command = self.form.command();

        self.redirect = Some(
            ListWorkspaceCommandsParams {
                workspace_id: command.workspace_id,
                search_query: "".into(),
                page_number: NonZeroU32::new(1),
                page_size: NonZeroU32::new(LIST_WORKSPACE_COMMANDS_PAGE_SIZE),
                powershell_no_exit: false,
            }
            .into(),
        );
    }

    pub fn new(parameters: NewWorkspaceCommandModelParameters) -> Result<Self> {
        let NewWorkspaceCommandModelParameters { workspace, theme } = parameters;

        let status_bar = StatusBar::builder()
            .operation("New command")
            .workspace(&workspace.name)
            .build();

        Ok(Self {
            form: CommandForm::new(NewCommandFormParameters {
                workspace_id: workspace.id,
                theme,
            }),
            redirect: None,
            status_bar,
            theme,
        })
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
        let CommandPresenter {
            id: _,
            name,
            program,
            workspace_id,
        } = self.form.command();

        self.redirect = Some(
            CreateWorkspaceCommandParams {
                workspace_id,
                name,
                program,
            }
            .into(),
        );
    }
}
