use crate::{
    components, parameters,
    presenters::workspace::Presenter,
    routes::{self, Route},
    tui, widgets, Message, Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::Paragraph,
    Frame,
};

pub struct Model {
    is_running: bool,
    redirect: Option<Route>,
    search: widgets::input::State,
    workspaces_state: widgets::list::State,
    workspaces: Vec<Presenter>,
    command_palette: components::command_palette::Component,
}

pub struct ModelParameters {
    pub workspaces: Vec<Presenter>,
    pub search_query: String,
}

impl tui::Model for Model {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        tui::EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ToggleCommandPalette => self.toggle_command_palette(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Back => self.back(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit(),
            Message::Action | Message::ToggleFocus => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, search_area, commands_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("List workspaces").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let input = widgets::input::Widget { title: "Search" };
        frame.render_stateful_widget(input, search_area, &mut self.search);
        frame.set_cursor_position(Position::new(
            search_area.x + self.search.character_index() as u16 + 1,
            search_area.y + 1,
        ));

        let list = widgets::list::Widget {
            title: "Workspaces",
            items: &self.workspaces,
        };

        frame.render_stateful_widget(list, commands_area, &mut self.workspaces_state);

        if self.command_palette.is_active() {
            self.command_palette.render(frame, frame.area());
        }
    }
}

impl Model {
    fn back(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.toggle();

            return;
        }

        self.is_running = false;
    }

    fn handle_command_palette_action(&mut self) {
        use components::command_palette::Action;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        if let Action::NewWorkspace = action {
            self.redirect = Some(Route::Workspaces(routes::workspaces::Route::New))
        }
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        use components::command_palette::Action;

        let ModelParameters {
            workspaces,
            search_query,
        } = parameters;

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: widgets::list::State::default(),
            search: widgets::input::State::new(widgets::input::StateParameters {
                value: search_query,
                is_active: true,
            }),
            is_running: true,
            command_palette: components::command_palette::Component::new(
                components::command_palette::ComponentParameters {
                    actions: vec![Action::NewWorkspace],
                },
            )?,
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        Ok(model)
    }

    fn submit(&mut self) {
        if self.command_palette.is_active() {
            self.handle_command_palette_action();

            return;
        }

        let maybe_workspace = self
            .workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i));

        let Some(workspace) = maybe_workspace else {
            return;
        };

        let route = Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: workspace.id.clone(),
                    ..Default::default()
                },
            ),
        ));

        self.redirect = Some(route);
    }

    fn select_next(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_next();
        } else {
            self.workspaces_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_previous();
        } else {
            self.workspaces_state.select_previous();
        }
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
            },
        ));

        self.redirect = Some(route);
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
            },
        ));

        self.redirect = Some(route);
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters {
                search_query: self.search_query(),
            },
        ));

        self.redirect = Some(route);
    }

    fn move_cursor_left(&mut self) {
        self.search.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.search.move_cursor_right();
    }

    fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
    }
}
