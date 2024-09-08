use super::elements::Input;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    name: Input,
    state: State,
}

pub enum State {
    Edit,
    Submited,
    Exited,
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

pub enum Message {
    MoveCusorLeft,
    MoveCusorRight,
    EnterChar(char),
    DeleteChar,
    Submit,
    Exit,
}

impl Model {
    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn new() -> Self {
        Self {
            name: Default::default(),
            state: State::Edit,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::DeleteChar => self.delete_char(),
            Message::Submit => self.submit(),
            Message::Exit => self.exit(),
        }
    }

    pub fn is_submited(&self) -> bool {
        matches!(self.state, State::Submited)
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn submit(&mut self) {
        self.state = State::Submited;
    }

    fn exit(&mut self) {
        self.state = State::Exited;
    }

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

    pub fn view(&self, frame: &mut Frame) {
        let view = View { name: &self.name };
        view.render(frame);
    }
}
