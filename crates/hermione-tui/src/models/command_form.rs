use super::{elements::Input, NewCommand};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    name: Input,
    program: Input,
    state: State,
    active_input: ActiveInput,
}

enum ActiveInput {
    Name,
    Program,
}

enum State {
    Edit,
    Submited,
    Exited,
}

struct View<'a> {
    name: &'a Input,
    program: &'a Input,
    active_input: &'a ActiveInput,
}

pub enum Message {
    MoveCusorLeft,
    MoveCusorRight,
    EnterChar(char),
    DeleteChar,
    ToggleActiveInput,
    Submit,
    Exit,
}

impl Model {
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            program: Default::default(),
            state: State::Edit,
            active_input: ActiveInput::Program,
        }
    }

    pub fn new_command(&self) -> NewCommand {
        NewCommand {
            name: self.name.value().to_string(),
            program: self.program.value().to_string(),
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
            Message::ToggleActiveInput => self.toggle_active_input(),
        }
    }

    fn toggle_active_input(&mut self) {
        self.active_input.toggle();
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
        match self.active_input {
            ActiveInput::Name => self.name.move_cursor_left(),
            ActiveInput::Program => self.program.move_cursor_left(),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.active_input {
            ActiveInput::Name => self.name.move_cursor_right(),
            ActiveInput::Program => self.program.move_cursor_right(),
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        match self.active_input {
            ActiveInput::Name => self.name.enter_char(new_char),
            ActiveInput::Program => self.program.enter_char(new_char),
        }
    }

    pub fn delete_char(&mut self) {
        match self.active_input {
            ActiveInput::Name => self.name.delete_char(),
            ActiveInput::Program => self.program.delete_char(),
        }
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            name: &self.name,
            program: &self.program,
            active_input: &self.active_input,
        };
        view.render(frame);
    }
}

impl<'a> View<'a> {
    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Min(3), Constraint::Min(3)],
        )
        .flex(Flex::Start);

        let [directive_input, name_input] = layout.areas(frame.area());

        let paragraph = Paragraph::new(self.program.value()).block(
            Block::new()
                .title("Enter directive")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );

        frame.render_widget(paragraph, directive_input);

        let paragraph = Paragraph::new(self.name.value()).block(
            Block::new()
                .title("Enter command name")
                .title_alignment(Alignment::Center)
                .borders(Borders::all()),
        );
        frame.render_widget(paragraph, name_input);

        let position = match self.active_input {
            ActiveInput::Name => Position::new(
                name_input.x + self.name.character_index() as u16 + 1,
                name_input.y + 1,
            ),
            ActiveInput::Program => Position::new(
                directive_input.x + self.program.character_index() as u16 + 1,
                directive_input.y + 1,
            ),
        };

        frame.set_cursor_position(position);
    }
}

impl ActiveInput {
    fn toggle(&mut self) {
        *self = match *self {
            ActiveInput::Name => ActiveInput::Program,
            ActiveInput::Program => ActiveInput::Name,
        };
    }
}
