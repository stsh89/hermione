use crate::{
    forms::CommandForm,
    layouts::{self, StatusBar},
    CommandPresenter, ListWorkspaceCommandsParams, Message, Result, Route,
    UpdateWorkspaceCommandParams, WorkspacePresenter, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{widgets::Paragraph, Frame};

pub struct EditWorkspaceCommandModel {
    status_bar: String,
    form: CommandForm,
    redirect: Option<Route>,
}

pub struct EditWorkspaceCommandModelParameters {
    pub command: CommandPresenter,
    pub workspace: WorkspacePresenter,
}

impl Model for EditWorkspaceCommandModel {
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
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());

        self.form.render(frame, main_area);

        let paragraph = Paragraph::new(self.status_bar.as_str());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl EditWorkspaceCommandModel {
    fn back(&mut self) {
        let command = self.form.command();

        self.redirect = Some(
            ListWorkspaceCommandsParams {
                workspace_id: command.workspace_id,
                search_query: command.program,
                page_number: 0,
                page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                powershell_no_exit: false,
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

    pub fn new(parameters: EditWorkspaceCommandModelParameters) -> Result<Self> {
        let EditWorkspaceCommandModelParameters { command, workspace } = parameters;

        let status_bar = StatusBar::default()
            .operation("Edit command")
            .workspace(&workspace.name)
            .command(&command.name)
            .try_into()?;

        Ok(Self {
            status_bar,
            redirect: None,
            form: command.into(),
        })
    }

    fn submit(&mut self) {
        let CommandPresenter {
            id,
            name,
            program,
            workspace_id,
        } = self.form.command();

        self.redirect = Some(
            UpdateWorkspaceCommandParams {
                name,
                program,
                workspace_id,
                command_id: id,
            }
            .into(),
        );
    }

    fn toggle_focus(&mut self) {
        self.form.select_next_input();
    }
}
