use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::elements::Input;

pub struct Model {
    form_input: FormInput,
    name: Input,
    program: Input,
    singnal: Option<Signal>,
}

enum FormInput {
    Name,
    Program,
}

pub enum Signal {
    Exit,
    CreateNewCommand(NewCommandParameters),
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
}

pub enum Message {
    DeleteChar,
    EnterChar(char),
    Exit,
    MoveCusorLeft,
    MoveCusorRight,
    Submit,
    ToggleFormInput,
}

struct View<'a> {
    name: &'a Input,
    program: &'a Input,
    active_input: &'a FormInput,
}

impl FormInput {
    fn toggle(&mut self) {
        *self = match *self {
            FormInput::Name => FormInput::Program,
            FormInput::Program => FormInput::Name,
        };
    }
}

impl Model {
    pub fn delete_char(&mut self) {
        match self.form_input {
            FormInput::Name => self.name.delete_char(),
            FormInput::Program => self.program.delete_char(),
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        match self.form_input {
            FormInput::Name => self.name.enter_char(new_char),
            FormInput::Program => self.program.enter_char(new_char),
        }
    }

    pub fn is_running(&self) -> bool {
        self.singnal.is_none()
    }

    pub fn move_cursor_left(&mut self) {
        match self.form_input {
            FormInput::Name => self.name.move_cursor_left(),
            FormInput::Program => self.program.move_cursor_left(),
        }
    }

    pub fn move_cursor_right(&mut self) {
        match self.form_input {
            FormInput::Name => self.name.move_cursor_right(),
            FormInput::Program => self.program.move_cursor_right(),
        }
    }

    pub fn new() -> Self {
        Self {
            form_input: FormInput::Program,
            name: Default::default(),
            program: Default::default(),
            singnal: None,
        }
    }

    pub unsafe fn signal(self) -> Signal {
        self.singnal.unwrap()
    }

    fn toggle_form_input(&mut self) {
        self.form_input.toggle();
    }

    pub fn update(mut self, message: Message) -> Self {
        match message {
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Exit => self.singnal = Some(Signal::Exit),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => {
                self.singnal = Some(Signal::CreateNewCommand(NewCommandParameters {
                    name: self.name.value().into(),
                    program: self.program.value().into(),
                }))
            }
            Message::ToggleFormInput => self.toggle_form_input(),
        }

        self
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            name: &self.name,
            program: &self.program,
            active_input: &self.form_input,
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
            FormInput::Name => Position::new(
                name_input.x + self.name.character_index() as u16 + 1,
                name_input.y + 1,
            ),
            FormInput::Program => Position::new(
                directive_input.x + self.program.character_index() as u16 + 1,
                directive_input.y + 1,
            ),
        };

        frame.set_cursor_position(position);
    }
}
