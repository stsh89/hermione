use crate::{elements::Input, Result};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Property {
    pub name: String,
    pub value: String,
}

pub struct Model {
    fields: Vec<Field>,
    current: usize,
}

struct Field {
    name: String,
    input: Input,
}

pub enum Message {
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    Exit,
    MoveCusorLeft,
    MoveCusorRight,
    Submit,
    Toggle,
}

pub enum Signal {
    Exit,
    Submit(Vec<Property>),
}

struct View<'a> {
    fields: &'a [Field],
    current: usize,
}

impl Model {
    fn delete_all_chars(&mut self) {
        self.input().delete_all_chars();
    }

    fn delete_char(&mut self) {
        self.input().delete_char();
    }

    fn enter_char(&mut self, new_char: char) {
        self.input().enter_char(new_char);
    }

    fn input(&mut self) -> &mut Input {
        &mut self.fields[self.current].input
    }

    pub fn new(properties: Vec<Property>) -> Result<Self> {
        if properties.is_empty() {
            return Err(anyhow::anyhow!("At least one input must be provided"));
        }

        let fields = properties
            .into_iter()
            .map(|property| Field {
                name: property.name,
                input: Input::new(property.value),
            })
            .collect();

        Ok(Self { fields, current: 0 })
    }

    fn move_cursor_left(&mut self) {
        self.input().move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.input().move_cursor_right();
    }

    fn toggle(&mut self) {
        if self.current == self.fields.len() - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
    }

    fn properties(&self) -> Vec<Property> {
        self.fields
            .iter()
            .map(|field| Property {
                name: field.name.clone(),
                value: field.input.value().into(),
            })
            .collect()
    }

    pub fn view(&self, frame: &mut Frame) {
        let view = View {
            fields: &self.fields,
            current: self.current,
        };
        view.render(frame);
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Signal>> {
        match message {
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Toggle => self.toggle(),
            Message::Exit => return Ok(Some(Signal::Exit)),
            Message::Submit => return Ok(Some(Signal::Submit(self.properties()))),
        };

        Ok(None)
    }
}

impl<'a> View<'a> {
    fn render(&self, frame: &mut Frame) {
        let constraints: Vec<Constraint> =
            (0..self.fields.len()).map(|_| Constraint::Max(3)).collect();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        for (index, field) in self.fields.iter().enumerate() {
            let paragraph = Paragraph::new(field.input.value()).block(
                Block::new()
                    .title(field.name.as_str())
                    .title_alignment(Alignment::Center)
                    .borders(Borders::all()),
            );

            frame.render_widget(paragraph, layout[index]);

            if index == self.current {
                frame.set_cursor_position(Position::new(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    layout[index].x + self.fields[index].input.character_index() as u16 + 1,
                    // Move one line down, from the border to the input line
                    layout[index].y + 1,
                ));
            }
        }
    }
}
