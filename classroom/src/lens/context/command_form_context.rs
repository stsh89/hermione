use crate::lens::input::Input;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct CommandFormContext {
    pub workspace_index: usize,
    pub name: Input,
    pub directive: Input,
    pub active_input: ActiveInput,
}

pub enum ActiveInput {
    Name,
    Directive,
}

impl CommandFormContext {
    pub fn toggle_active_input(&mut self) {
        self.active_input = match self.active_input {
            ActiveInput::Name => ActiveInput::Directive,
            ActiveInput::Directive => ActiveInput::Name,
        };
    }

    pub fn move_cursor_left(&mut self) {
        match self.active_input {
            ActiveInput::Name => self.name.move_cursor_left(),
            ActiveInput::Directive => self.directive.move_cursor_left(),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.active_input {
            ActiveInput::Name => self.name.move_cursor_right(),
            ActiveInput::Directive => self.directive.move_cursor_right(),
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        match self.active_input {
            ActiveInput::Name => self.name.enter_char(new_char),
            ActiveInput::Directive => self.directive.enter_char(new_char),
        }
    }

    pub fn delete_char(&mut self) {
        match self.active_input {
            ActiveInput::Name => self.name.delete_char(),
            ActiveInput::Directive => self.directive.delete_char(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Min(3), Constraint::Min(3)],
        )
        .flex(Flex::Start);

        let [directive_input, name_input] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.directive.value.as_str()).block(
            Block::new()
                .title("Enter directive")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );

        frame.render_widget(paragraph, directive_input);

        let paragraph = Paragraph::new(self.name.value.as_str()).block(
            Block::new()
                .title("Enter command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, name_input);

        let position = match self.active_input {
            ActiveInput::Name => Position::new(
                name_input.x + self.name.character_index as u16 + 1,
                name_input.y + 1,
            ),
            ActiveInput::Directive => Position::new(
                directive_input.x + self.directive.character_index as u16 + 1,
                directive_input.y + 1,
            ),
        };

        frame.set_cursor_position(position);
    }
}
