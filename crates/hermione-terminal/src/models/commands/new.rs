use crate::{
    forms::CommandForm,
    layouts::{self, StatusBar},
    CommandPresenter, CreateWorkspaceCommandParams, ListWorkspaceCommandsParams, Message, Result,
    Route, WorkspacePresenter, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{widgets::Paragraph, Frame};

pub struct NewWorkspaceCommandModel {
    status_bar: String,
    form: CommandForm,
    redirect: Option<Route>,
}

pub struct NewWorkspaceCommandModelParameters {
    pub workspace: WorkspacePresenter,
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
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());

        self.form.render(frame, main_area);

        let paragraph = Paragraph::new(self.status_bar.as_str());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl NewWorkspaceCommandModel {
    fn back(&mut self) {
        let command = self.form.command();

        self.redirect = Some(
            ListWorkspaceCommandsParams {
                workspace_id: command.workspace_id,
                search_query: "".into(),
                page_number: 0,
                page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                powershell_no_exit: false,
            }
            .into(),
        );
    }

    pub fn new(parameters: NewWorkspaceCommandModelParameters) -> Result<Self> {
        let NewWorkspaceCommandModelParameters { workspace } = parameters;

        let status_bar = StatusBar::default()
            .operation("New command")
            .workspace(&workspace.name)
            .try_into()?;

        let command = CommandPresenter {
            id: String::new(),
            name: String::new(),
            program: String::new(),
            workspace_id: workspace.id,
        };

        Ok(Self {
            form: command.into(),
            redirect: None,
            status_bar,
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
