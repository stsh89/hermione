use crate::{
    forms,
    layouts::{self, StatusBar},
    parameters, presenters,
    routes::Route,
    Message, Result,
};
use hermione_tui::app::{self, EventHandler};
use ratatui::{widgets::Paragraph, Frame};

pub struct Model {
    status_bar: String,
    form: forms::command::Form,
    redirect: Option<Route>,
}

pub struct ModelParameters {
    pub command: presenters::command::Presenter,
    pub workspace: presenters::workspace::Presenter,
}

impl app::Model for Model {
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
            Message::ToggleFocus => self.toggle_focus(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            Message::Action | Message::SelectNext | Message::SelectPrevious => {}
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

impl Model {
    fn back(&mut self) {
        let command = self.form.command();

        self.redirect = Some(
            parameters::workspaces::commands::list::Parameters {
                workspace_id: command.workspace_id,
                search_query: command.program,
                page_number: 0,
                page_size: parameters::workspaces::commands::list::PAGE_SIZE,
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

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters { command, workspace } = parameters;

        let status_bar = StatusBar::default()
            .use_case("Edit command")
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
        let presenters::command::Presenter {
            id,
            name,
            program,
            workspace_id,
        } = self.form.command();

        self.redirect = Some(
            parameters::workspaces::commands::update::Parameters {
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
