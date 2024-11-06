use std::num::NonZeroU32;

use crate::{
    forms::{CommandForm, EditCommandFormParameters},
    layouts::WideLayout,
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    CommandPresenter, ListWorkspaceCommandsParams, Message, Result, Route,
    UpdateWorkspaceCommandParams, WorkspacePresenter, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::Frame;

pub struct EditWorkspaceCommandModel {
    status_bar: StatusBar,
    form: CommandForm,
    redirect: Option<Route>,
    theme: Theme,
}

pub struct EditWorkspaceCommandModelParameters {
    pub command: CommandPresenter,
    pub workspace: WorkspacePresenter,
    pub theme: Theme,
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
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());

        self.form.render(frame, main_area);

        frame.render_widget(
            StatusBarWidget::new(&self.status_bar).themed(self.theme),
            status_bar_area,
        );
    }
}

impl EditWorkspaceCommandModel {
    fn back(&mut self) {
        let command = self.form.command();

        self.redirect = Some(
            ListWorkspaceCommandsParams {
                workspace_id: command.workspace_id,
                search_query: command.program,
                page_number: NonZeroU32::new(1),
                page_size: NonZeroU32::new(LIST_WORKSPACE_COMMANDS_PAGE_SIZE),
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
        let EditWorkspaceCommandModelParameters {
            command,
            workspace,
            theme,
        } = parameters;

        let status_bar = StatusBar::builder()
            .operation("Edit command")
            .workspace(&workspace.name)
            .command(&command.name)
            .build();

        Ok(Self {
            status_bar,
            redirect: None,
            form: CommandForm::edit(EditCommandFormParameters { command, theme }),
            theme,
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
