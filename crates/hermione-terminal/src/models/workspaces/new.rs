use crate::{
    parameters,
    routes::{self, Route},
    tui, widgets, Message, Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    widgets::Paragraph,
    Frame,
};

pub struct Model {
    active_input: WorkspaceProperty,
    location: widgets::input::State,
    name: widgets::input::State,
    redirect: Option<Route>,
}

enum WorkspaceProperty {
    Name,
    Location,
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
        let [header, name_area, location_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("New workspace").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        for (area, property) in [
            (name_area, WorkspaceProperty::Name),
            (location_area, WorkspaceProperty::Location),
        ] {
            self.render_property(frame, area, property);
        }
    }
}

impl Model {
    fn back(&mut self) {
        let route = Route::Workspaces(routes::workspaces::Route::List(
            parameters::workspaces::list::Parameters::default(),
        ));

        self.redirect = Some(route);
    }

    fn delete_char(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.delete_char(),
            WorkspaceProperty::Location => self.location.delete_char(),
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.delete_all_chars(),
            WorkspaceProperty::Location => self.location.delete_all_chars(),
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.enter_char(c),
            WorkspaceProperty::Location => self.location.enter_char(c),
        }
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.move_cursor_left(),
            WorkspaceProperty::Location => self.location.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.move_cursor_right(),
            WorkspaceProperty::Location => self.location.move_cursor_right(),
        }
    }

    pub fn new() -> Self {
        Self {
            name: widgets::input::State::new(widgets::input::StateParameters {
                value: String::new(),
                is_active: true,
            }),
            redirect: None,
            active_input: WorkspaceProperty::Name,
            location: widgets::input::State::new(widgets::input::StateParameters {
                value: String::new(),
                is_active: false,
            }),
        }
    }

    fn render_property(&mut self, frame: &mut Frame, area: Rect, property: WorkspaceProperty) {
        let title = match property {
            WorkspaceProperty::Name => "Name",
            WorkspaceProperty::Location => "Location",
        };

        let input = widgets::input::Widget { title };

        let state = match property {
            WorkspaceProperty::Name => &mut self.name,
            WorkspaceProperty::Location => &mut self.location,
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
        let route = Route::Workspaces(routes::workspaces::Route::Create(
            parameters::workspaces::create::Parameters {
                name: self.name.value().to_string(),
                location: self.location.value().to_string(),
            },
        ));

        self.redirect = Some(route);
    }

    fn toggle_focus(&mut self) {
        self.active_input = match self.active_input {
            WorkspaceProperty::Name => WorkspaceProperty::Location,
            WorkspaceProperty::Location => WorkspaceProperty::Name,
        };

        match self.active_input {
            WorkspaceProperty::Name => {
                self.name.activate();
                self.location.deactivate();
            }
            WorkspaceProperty::Location => {
                self.location.activate();
                self.name.deactivate();
            }
        }
    }
}
