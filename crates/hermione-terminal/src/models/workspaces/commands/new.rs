use crate::{
    layouts::{self, Breadcrumbs},
    parameters,
    presenters::workspace::Presenter,
    routes::{self, Route},
    tui, widgets, Message, Result,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Position, Rect},
    widgets::Paragraph,
    Frame,
};

pub struct Model {
    workspace: Presenter,
    name: widgets::input::State,
    program: widgets::input::State,
    redirect: Option<Route>,
    active_input: CommandProperty,
}

pub struct ModelParameters {
    pub workspace: Presenter,
}

enum CommandProperty {
    Name,
    Program,
}

impl tui::Model for Model {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        tui::EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            Message::ToggleFocus => self.toggle_focus(),
            Message::Action
            | Message::SelectNext
            | Message::SelectPrevious
            | Message::ActivateCommandPalette => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::full_width::Layout::new().areas(frame.area());

        let [name_area, program_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .areas(main_area);

        for (area, property) in [
            (name_area, CommandProperty::Name),
            (program_area, CommandProperty::Program),
        ] {
            self.render_property(frame, area, property);
        }

        let paragraph = Paragraph::new(self.breadcrumbs());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl Model {
    fn back(&mut self) {
        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: self.workspace.id.clone(),
                    search_query: "".into(),
                    page_number: 0,
                    page_size: parameters::workspaces::commands::list::PAGE_SIZE,
                },
            ),
        ));

        self.redirect = Some(route);
    }

    fn breadcrumbs(&self) -> Breadcrumbs {
        Breadcrumbs::default()
            .add_segment("List workspaces")
            .add_segment(&self.workspace.name)
            .add_segment("New command")
    }

    pub fn new(parameters: ModelParameters) -> Self {
        let ModelParameters { workspace } = parameters;

        Self {
            workspace,
            name: widgets::input::State::new(widgets::input::StateParameters {
                value: String::new(),
                is_active: true,
            }),
            program: widgets::input::State::new(widgets::input::StateParameters {
                value: String::new(),
                is_active: false,
            }),
            redirect: None,
            active_input: CommandProperty::Name,
        }
    }

    fn toggle_focus(&mut self) {
        self.active_input = match self.active_input {
            CommandProperty::Name => CommandProperty::Program,
            CommandProperty::Program => CommandProperty::Name,
        };

        match self.active_input {
            CommandProperty::Name => {
                self.program.deactivate();
                self.name.activate();
            }
            CommandProperty::Program => {
                self.name.deactivate();
                self.program.activate();
            }
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            CommandProperty::Name => self.name.enter_char(c),
            CommandProperty::Program => self.program.enter_char(c),
        }
    }

    fn delete_char(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_char(),
            CommandProperty::Program => self.program.delete_char(),
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_all_chars(),
            CommandProperty::Program => self.program.delete_all_chars(),
        }
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_left(),
            CommandProperty::Program => self.program.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_right(),
            CommandProperty::Program => self.program.move_cursor_right(),
        }
    }

    fn render_property(&mut self, frame: &mut Frame, area: Rect, property: CommandProperty) {
        let title = match property {
            CommandProperty::Name => "Name",
            CommandProperty::Program => "Program",
        };

        let input = widgets::input::Widget { title };

        let state = match property {
            CommandProperty::Name => &mut self.name,
            CommandProperty::Program => &mut self.program,
        };

        frame.render_stateful_widget(input, area, state);

        if state.is_active() {
            frame.set_cursor_position(Position::new(
                area.x + state.character_index() as u16 + 1,
                area.y + 1,
            ));
        }
    }

    fn submit(&mut self) {
        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::Create(
                parameters::workspaces::commands::create::Parameters {
                    workspace_id: self.workspace.id.clone(),
                    name: self.name.value().to_string(),
                    program: self.program.value().to_string(),
                },
            ),
        ));

        self.redirect = Some(route);
    }
}
