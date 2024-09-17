use crate::{elements::Input, key_mappings::InputMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    name: Input,
    signal: Option<Signal>,
}

pub enum Signal {
    Exit,
    ChangeLocation(String),
}

pub enum Message {
    DeleteChar,
    EnterChar(char),
    Exit,
    MoveCusorLeft,
    MoveCusorRight,
    Submit,
}

impl Model {
    fn delete_char(&mut self) {
        self.name.delete_char();
    }

    fn enter_char(&mut self, new_char: char) {
        self.name.enter_char(new_char);
    }

    pub fn input_mode(&self) -> InputMode {
        InputMode::Editing
    }

    pub fn is_running(&self) -> bool {
        self.signal.is_none()
    }

    fn move_cursor_left(&mut self) {
        self.name.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.name.move_cursor_right();
    }

    pub fn new(location: String) -> Self {
        Self {
            name: Input::new(location),
            signal: None,
        }
    }

    pub unsafe fn signal(self) -> Signal {
        self.signal.unwrap()
    }

    pub fn update(mut self, message: Message) -> Self {
        match message {
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Exit => self.signal = Some(Signal::Exit),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.signal = Some(Signal::ChangeLocation(self.name.value().into())),
        }

        self
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View { name: &self.name };
        view.render(frame);
    }
}

struct View<'a> {
    name: &'a Input,
}

impl<'a> View<'a> {
    fn render(&self, frame: &mut Frame) {
        let layout =
            Layout::new(Direction::Vertical, vec![Constraint::Percentage(100)]).flex(Flex::Start);

        let [top] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.name.value()).block(
            Block::new()
                .title("Enter workspace name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );

        frame.render_widget(paragraph, top);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            top.x + self.name.character_index() as u16 + 1,
            // Move one line down, from the border to the input line
            top.y + 1,
        ));
    }
}
