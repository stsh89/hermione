use crate::{
    layouts::WideLayout, themes::Theme, widgets::FormField, CreateWorkspaceParams,
    ListWorkspacesParams, Message, Result, Route, UpdateWorkspaceParams, WorkspacePresenter,
};
use hermione_tui::{EventHandler, Input, Model};
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    style::Stylize,
    text::ToText,
    widgets::Paragraph,
    Frame,
};
use uuid::Uuid;

#[derive(Default)]
pub struct WorkspaceModel {
    active_input: InputName,
    location: Input,
    name: Input,
    redirect: Option<Route>,
    theme: Theme,
    id: Option<Uuid>,
}

#[derive(Default, PartialEq)]
enum InputName {
    #[default]
    Name,
    Location,
}

impl Model for WorkspaceModel {
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
        let input_areas = input_areas(main_area);

        self.render_name(frame, input_areas[0]);
        self.render_location(frame, input_areas[1]);
        self.render_status_bar(frame, status_bar_area);

        self.set_cursor_position(frame, input_areas[self.active_input.as_index()]);
    }
}

impl WorkspaceModel {
    fn active_input(&self) -> &Input {
        match self.active_input {
            InputName::Name => &self.name,
            InputName::Location => &self.location,
        }
    }

    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            InputName::Name => &mut self.name,
            InputName::Location => &mut self.location,
        }
    }

    fn back(&mut self) {
        self.set_redirect(
            ListWorkspacesParams {
                search_query: String::new(),
                page_number: None,
                page_size: None,
            }
            .into(),
        );
    }

    fn create_workspace_parameters(&self) -> CreateWorkspaceParams {
        CreateWorkspaceParams {
            name: self.name.value().to_string(),
            location: self.location.value().to_string(),
        }
    }

    fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    fn delete_all_chars(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    pub fn workspace(self, workspace: WorkspacePresenter) -> Self {
        let WorkspacePresenter { id, location, name } = workspace;

        Self {
            name: Input::new(name),
            location: Input::new(location),
            id: Some(id),
            ..self
        }
    }

    pub fn theme(self, theme: Theme) -> Self {
        Self { theme, ..self }
    }

    fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    fn render_location(&self, frame: &mut Frame, area: Rect) {
        let mut field = FormField::default()
            .name(InputName::Location.as_str())
            .value(self.location.value())
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::Location == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_name(&self, frame: &mut Frame, area: Rect) {
        let mut field = FormField::default()
            .name(InputName::Name.as_str())
            .value(self.name.value())
            .set_background_color(self.theme.background_color)
            .set_foreground_color(self.theme.foreground_color);

        if InputName::Name == self.active_input {
            field = field.set_foreground_color(self.theme.input_color);
        }

        frame.render_widget(field, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let screen = if self.id.is_some() {
            "Edit workspace"
        } else {
            "New workspace"
        };

        let value = serde_json::json!({
            "screen": screen,
        });

        let paragraph = Paragraph::new(value.to_text())
            .bg(self.theme.status_bar_background_color)
            .fg(self.theme.status_bar_foreground_color);

        frame.render_widget(paragraph, area);
    }

    fn set_cursor_position(&self, frame: &mut Frame, area: Rect) {
        frame.set_cursor_position(Position::new(
            area.x + self.active_input().character_index() as u16 + 1,
            area.y + 1,
        ));
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn submit(&mut self) {
        let route = if let Some(id) = self.id {
            self.update_workspace_parameters(id).into()
        } else {
            self.create_workspace_parameters().into()
        };

        self.set_redirect(route);
    }

    fn toggle_focus(&mut self) {
        self.active_input = self.active_input.next();
    }

    fn update_workspace_parameters(&self, id: Uuid) -> UpdateWorkspaceParams {
        UpdateWorkspaceParams {
            name: self.name.value().to_string(),
            location: self.location.value().to_string(),
            id,
        }
    }
}

impl InputName {
    fn as_index(&self) -> usize {
        match self {
            InputName::Name => 0,
            InputName::Location => 1,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            InputName::Name => "Name",
            InputName::Location => "Location",
        }
    }

    fn next(&self) -> Self {
        match self {
            InputName::Name => InputName::Location,
            InputName::Location => InputName::Name,
        }
    }
}

fn input_areas(main_area: Rect) -> [Rect; 2] {
    ratatui::layout::Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
        .areas(main_area)
}
