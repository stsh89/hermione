use crate::{
    breadcrumbs::Breadcrumbs, forms, layouts, parameters, presenters, routes, Message, Result,
};
use hermione_tui::app::{self, EventHandler};
use ratatui::{widgets::Paragraph, Frame};

pub struct Model {
    form: forms::workspace::Form,
    breadcrumbs: String,
    redirect: Option<routes::Route>,
}

pub struct ModelParameters {
    pub workspace: presenters::workspace::Presenter,
}

impl app::Model for Model {
    type Message = Message;
    type Route = routes::Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn redirect(&mut self) -> Option<routes::Route> {
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
            Message::Action
            | Message::SelectNext
            | Message::SelectPrevious
            | Message::ActivateCommandPalette => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());

        self.form.render(frame, main_area);

        let paragraph = Paragraph::new(self.breadcrumbs.as_str());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl Model {
    fn back(&mut self) {
        use parameters::workspaces::list;

        let presenters::workspace::Presenter {
            id: _,
            name: search_query,
            location: _,
        } = self.form.workspace();

        self.redirect = Some(
            list::Parameters {
                search_query,
                page_number: 0,
                page_size: list::PAGE_SIZE,
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

    pub fn new(parameters: ModelParameters) -> Self {
        let ModelParameters { workspace } = parameters;

        let breadcrumbs = Breadcrumbs::default()
            .add_segment("List workspaces")
            .add_segment(&workspace.name)
            .add_segment("Edit workspace")
            .to_string();

        Self {
            redirect: None,
            breadcrumbs,
            form: workspace.into(),
        }
    }

    fn submit(&mut self) {
        use parameters::workspaces::update::Parameters;

        let presenters::workspace::Presenter { id, name, location } = self.form.workspace();

        self.redirect = Some(Parameters { name, location, id }.into());
    }

    fn toggle_focus(&mut self) {
        self.form.select_next_input();
    }
}
