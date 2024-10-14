use crate::inputs::{
    base::BaseInput,
    command::{CommandInput, NewCommandInputParameters},
};
use hermione_tui::input::Input;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const COMMAND_PREFIX: char = '>';

pub struct SmartInput {
    commands: Vec<String>,
    inner: Box<dyn InputContract>,
    mode: Mode,
}

pub trait InputContract {
    fn delete_char(&mut self);
    fn enter_char(&mut self, c: char);
    fn input(&self) -> &Input;
    fn is_empty(&self) -> bool;
    fn move_cursor_left(&mut self) {}
    fn move_cursor_right(&mut self) {}
    fn reset(&mut self);
    fn toggle_input(&mut self) {}
    fn value(&self) -> Option<&str>;
}

pub enum Value<'a> {
    Base(&'a str),
    Command(&'a str),
}

#[derive(Default)]
enum Mode {
    #[default]
    Base,
    Command,
}

pub struct NewSmartInputParameters {
    pub commands: Vec<String>,
}

impl SmartInput {
    fn set_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.inner = Box::new(CommandInput::new(NewCommandInputParameters {
            commands: self.commands.clone(),
            prefix: COMMAND_PREFIX,
        }));
    }

    pub fn reset_input(&mut self) {
        self.inner.reset();
    }

    pub fn new(parameters: NewSmartInputParameters) -> Self {
        let NewSmartInputParameters { commands } = parameters;

        Self {
            commands,
            mode: Mode::Base,
            inner: Box::new(BaseInput::default()),
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("Console");
        let paragraph = Paragraph::new(self.inner.input().value()).block(block);

        self.inner.input().render(frame, area, paragraph);
    }

    pub fn delete_char(&mut self) {
        self.inner.delete_char();

        if self.inner.is_empty() && !matches!(self.mode, Mode::Base) {
            self.mode = Mode::Base;
            self.inner = Box::new(BaseInput::default());
        }
    }

    pub fn enter_char(&mut self, c: char) {
        if self.inner.is_empty() && c == COMMAND_PREFIX {
            self.set_command_mode();

            return;
        }

        self.inner.enter_char(c);
    }

    pub fn move_cursor_left(&mut self) {
        self.inner.move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.inner.move_cursor_right();
    }

    pub fn toggle_input(&mut self) {
        self.inner.toggle_input();
    }

    pub fn value(&self) -> Option<Value> {
        let value = self.inner.value();

        match self.mode {
            Mode::Base => value.map(Value::Base),
            Mode::Command => value.map(Value::Command),
        }
    }
}
