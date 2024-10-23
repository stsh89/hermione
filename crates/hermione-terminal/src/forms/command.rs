use crate::Command;
use hermione_tui::Input;
use ratatui::{
    layout::{Constraint, Direction, Position, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const NAME: &str = "Name";
const PROGRAM: &str = "Program";

#[derive(Default)]
pub struct CommandForm {
    id: String,
    active_input: ActiveInput,
    program: Input,
    name: Input,
    workspace_id: String,
}

#[derive(Default)]
enum ActiveInput {
    #[default]
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

    pub fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    pub fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    pub fn select_next_input(&mut self) {
        self.active_input = self.active_input.next();
    }

    pub fn command(&self) -> Command {
        Command {
            id: self.id.clone(),
            name: self.name.value().into(),
            program: self.program.value().into(),
            workspace_id: self.workspace_id.clone(),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [name_area, program_area] = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .areas(area);

        let block = Block::default().borders(Borders::ALL).title(NAME);
        let paragraph = Paragraph::new(self.name.value()).block(block);
        frame.render_widget(paragraph, name_area);

        let block = Block::default().borders(Borders::ALL).title(PROGRAM);
        let paragraph = Paragraph::new(self.program.value()).block(block);
        frame.render_widget(paragraph, program_area);

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

impl From<Command> for CommandForm {
    fn from(command: Command) -> Self {
        let Command {
            id,
            name,
            program,
            workspace_id,
        } = command;

        Self {
            id,
            program: Input::new(program),
            name: Input::new(name),
            workspace_id,
            ..Default::default()
        }
    }
}
