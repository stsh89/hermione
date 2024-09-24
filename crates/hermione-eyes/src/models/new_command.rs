use crate::{
    models::{
        handle_event, highlight_style,
        shared::{Input, Menu, MenuItem},
        Message, Router,
    },
    router::{CreateCommandParameters, CreateWorkspaceParameters, GetWorkspaceParameters, ListWorkspacesParameters},
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::shared::InputParameters;

pub struct NewCommandModel {
    name: Input,
    program: Input,
    menu: Menu,
    redirect: Option<Router>,
    is_running: bool,
    active_input: CommandProperty,
}

enum CommandProperty {
    Name,
    Program,
}

impl NewCommandModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new() -> Self {
        Self {
            name: Input::new(InputParameters {
                value: String::new(),
                is_active: true,
            }),
            program: Input::new(InputParameters {
                value: String::new(),
                is_active: false,
            }),
            redirect: None,
            menu: Menu::new(vec![MenuItem::Back, MenuItem::Exit]),
            is_running: true,
            active_input: CommandProperty::Name,
        }
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let [header, name, program] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("New command").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Name");
        let paragraph = Paragraph::new(self.name.value()).block(block);

        frame.render_widget(paragraph, name);

        if self.name.is_active() {
            frame.set_cursor_position(Position::new(
                name.x + self.name.character_index() as u16 + 1,
                name.y + 1,
            ));
        }
        let block = Block::default().borders(Borders::all()).title("Program");
        let paragraph = Paragraph::new(self.program.value()).block(block);

        frame.render_widget(paragraph, program);

        if self.program.is_active() {
            frame.set_cursor_position(Position::new(
                program.x + self.program.character_index() as u16 + 1,
                program.y + 1,
            ));
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    fn toggle_forcus(&mut self) {
        self.active_input = match self.active_input {
            CommandProperty::Name => {
                self.name.deactivate();
                self.program.activate();

                CommandProperty::Program
            }
            CommandProperty::Program => {
                self.program.deactivate();
                self.name.activate();

                CommandProperty::Name
            }
        };
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            CommandProperty::Name => self.name.enter_char(c),
            CommandProperty::Program => self.program.enter_char(c),
        }
    }

    fn delete_char(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_char(),
            CommandProperty::Program => self.program.delete_char(),
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.delete_all_chars(),
            CommandProperty::Program => self.program.delete_all_chars(),
        }
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_left(),
            CommandProperty::Program => self.program.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            CommandProperty::Name => self.name.move_cursor_right(),
            CommandProperty::Program => self.program.move_cursor_right(),
        }
    }

    fn submit(&mut self) {
        self.redirect = Some(Router::CreateCommand(CreateCommandParameters {
            name: self.name.value().to_string(),
            program: self.program.value().to_string(),
        }));
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::EnterChar(c) => self.enter_char(c),
            Message::DeleteChar => self.delete_char(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::ToggleForcus => self.toggle_forcus(),
            Message::Back => {
                self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
                    number: 0,
                    commands_search_query: String::new(),
                }))
            }
            Message::Sumbit => self.submit(),
            _ => {}
        }

        Ok(None)
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Char(c) => Message::EnterChar(c),
        KeyCode::Backspace => match key_event.modifiers {
            KeyModifiers::CONTROL => Message::DeleteAllChars,
            _ => Message::DeleteChar,
        },
        KeyCode::Esc => Message::Back,
        KeyCode::Enter => Message::Sumbit,
        KeyCode::Left => Message::MoveCusorLeft,
        KeyCode::Right => Message::MoveCusorRight,
        KeyCode::Tab => Message::ToggleForcus,
        _ => return None,
    };

    Some(message)
}
