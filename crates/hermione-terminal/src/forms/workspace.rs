use crate::{
    themes::{Theme, Themed},
    widgets::TextInputWidget,
    WorkspacePresenter,
};
use hermione_tui::Input;
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use uuid::Uuid;

const NAME: &str = "Name";
const LOCATION: &str = "Location";

pub struct WorkspaceForm {
    id: Uuid,
    active_input: ActiveInput,
    location: Input,
    name: Input,
    theme: Theme,
}

pub struct EditWorkspaceFormParameters {
    pub workspace: WorkspacePresenter,
    pub theme: Theme,
}

pub struct NewWorkspaceFormParameters {
    pub theme: Theme,
}

enum ActiveInput {
    Name,
    Location,
}

impl WorkspaceForm {
    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            ActiveInput::Name => &mut self.name,
            ActiveInput::Location => &mut self.location,
        }
    }

    fn active_input(&self) -> &Input {
        match self.active_input {
            ActiveInput::Name => &self.name,
            ActiveInput::Location => &self.location,
        }
    }

    pub fn delete_all_chars(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    pub fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    pub fn edit(parameters: EditWorkspaceFormParameters) -> Self {
        let EditWorkspaceFormParameters { workspace, theme } = parameters;
        let WorkspacePresenter { id, location, name } = workspace;

        Self {
            id,
            active_input: ActiveInput::Name,
            location: Input::new(location),
            name: Input::new(name),
            theme,
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    fn location_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.location.value()).themed(self.theme)
    }

    fn location_text(&self) -> Paragraph {
        Paragraph::new(self.location.value())
    }

    pub fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    fn name_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.name.value()).themed(self.theme)
    }

    fn name_text(&self) -> Paragraph {
        Paragraph::new(self.name.value())
    }

    pub fn new(parameters: NewWorkspaceFormParameters) -> Self {
        let NewWorkspaceFormParameters { theme } = parameters;

        Self {
            id: Default::default(),
            active_input: ActiveInput::Name,
            location: Input::default(),
            name: Input::default(),
            theme,
        }
    }

    pub fn select_next_input(&mut self) {
        self.active_input = self.active_input.next();
    }

    pub fn workspace(&self) -> WorkspacePresenter {
        WorkspacePresenter {
            id: self.id,
            name: self.name.value().to_string(),
            location: self.location.value().to_string(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [name_area, location_area] = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .areas(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(NAME)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::Name => {
                frame.render_widget(self.name_input().block(block), name_area);
            }
            ActiveInput::Location => {
                frame.render_widget(self.name_text().block(block), name_area);
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(LOCATION)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::Name => {
                frame.render_widget(self.location_text().block(block), location_area)
            }
            ActiveInput::Location => {
                frame.render_widget(self.location_input().block(block), location_area)
            }
        };

        let active_input_area = match self.active_input {
            ActiveInput::Name => name_area,
            ActiveInput::Location => location_area,
        };

        frame.set_cursor_position(Position::new(
            active_input_area.x + self.active_input().character_index() as u16 + 1,
            active_input_area.y + 1,
        ));
    }
}

impl ActiveInput {
    fn next(&self) -> Self {
        match self {
            ActiveInput::Name => ActiveInput::Location,
            ActiveInput::Location => ActiveInput::Name,
        }
    }
}
