use crate::desktop::input::Input;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct WorkspaceFormContext {
    pub name: Input,
}

impl WorkspaceFormContext {
    pub fn move_cursor_left(&mut self) {
        self.name.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.name.move_cursor_right();
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.name.enter_char(new_char);
    }

    pub fn delete_char(&mut self) {
        self.name.delete_char();
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout =
            Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).flex(Flex::Start);

        let [top] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.name.value.as_str()).block(
            Block::new()
                .title("Enter workspace name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );

        frame.render_widget(paragraph, top);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            top.x + self.name.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            top.y + 1,
        ));
    }
}
