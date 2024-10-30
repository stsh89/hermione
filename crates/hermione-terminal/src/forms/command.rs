use crate::{
    themes::{Theme, Themed},
    widgets::TextInputWidget,
    CommandPresenter,
};
use hermione_tui::Input;
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const NAME: &str = "Name";
const PROGRAM: &str = "Program";

pub struct CommandForm {
    id: String,
    active_input: ActiveInput,
    program: Input,
    name: Input,
    workspace_id: String,
    theme: Theme,
}

pub struct EditCommandFormParameters {
    pub command: CommandPresenter,
    pub theme: Theme,
}

pub struct NewCommandFormParameters {
    pub theme: Theme,
    pub workspace_id: String,
}

enum ActiveInput {
    Name,
    Program,
}

impl CommandForm {
    fn active_input(&self) -> &Input {
        match self.active_input {
            ActiveInput::Name => &self.name,
            ActiveInput::Program => &self.program,
        }
    }

    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            ActiveInput::Name => &mut self.name,
            ActiveInput::Program => &mut self.program,
        }
    }

    pub fn delete_all_chars(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    pub fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    pub fn edit(parameters: EditCommandFormParameters) -> Self {
        let EditCommandFormParameters { command, theme } = parameters;
        let CommandPresenter {
            workspace_id,
            id,
            name,
            program,
        } = command;

        Self {
            id,
            active_input: ActiveInput::Name,
            program: Input::new(program),
            name: Input::new(name),
            workspace_id,
            theme,
        }
    }

    pub fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
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

    pub fn new(parameters: NewCommandFormParameters) -> Self {
        let NewCommandFormParameters {
            theme,
            workspace_id,
        } = parameters;

        Self {
            id: Default::default(),
            active_input: ActiveInput::Name,
            program: Input::default(),
            name: Input::default(),
            workspace_id,
            theme,
        }
    }

    pub fn select_next_input(&mut self) {
        self.active_input = self.active_input.next();
    }

    pub fn command(&self) -> CommandPresenter {
        CommandPresenter {
            id: self.id.clone(),
            name: self.name.value().into(),
            program: self.program.value().into(),
            workspace_id: self.workspace_id.clone(),
        }
    }

    fn program_input(&self) -> TextInputWidget {
        TextInputWidget::new(self.program.value()).themed(self.theme)
    }

    fn program_text(&self) -> Paragraph {
        Paragraph::new(self.program.value())
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [name_area, program_area] = ratatui::layout::Layout::default()
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
            ActiveInput::Program => {
                frame.render_widget(self.name_text().block(block), name_area);
            }
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(PROGRAM)
            .themed(self.theme);

        match self.active_input {
            ActiveInput::Name => {
                frame.render_widget(self.program_text().block(block), program_area);
            }
            ActiveInput::Program => {
                frame.render_widget(self.program_input().block(block), program_area);
            }
        };

        let active_input_area = match self.active_input {
            ActiveInput::Name => name_area,
            ActiveInput::Program => program_area,
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
            ActiveInput::Name => ActiveInput::Program,
            ActiveInput::Program => ActiveInput::Name,
        }
    }
}
