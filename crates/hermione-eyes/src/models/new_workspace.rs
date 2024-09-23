use crate::{
    models::{
        handle_event, highlight_style,
        shared::Input,
        shared::{Menu, MenuItem},
        Message, Redirect, Router,
    },
    router::CreateWorkspaceParameters,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub struct NewWorkspaceModel {
    input: Input,
    menu: Menu,
    status: Option<Status>,
}

enum Status {
    Back,
    Exit,
    Submit,
}

impl NewWorkspaceModel {
    pub fn new() -> Self {
        Self {
            input: Input::active(),
            status: None,
            menu: Menu::new(vec![MenuItem::Back, MenuItem::Exit]),
        }
    }

    pub fn redirect(&self) -> Option<Redirect> {
        self.status.as_ref().map(|status| match status {
            Status::Back => Redirect::Route(Router::ListWorkspaces),
            Status::Exit => Redirect::Exit,
            Status::Submit => Redirect::Route(Router::CreateWorkspace(CreateWorkspaceParameters {
                name: self.input.value().to_string(),
            })),
        })
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.area());

        let mut block = Block::default().borders(Borders::all());
        block = if self.menu.is_active() {
            block.border_style(highlight_style())
        } else {
            block
        };
        let items: Vec<ListItem> = self.menu.items().iter().map(ListItem::from).collect();
        let mut list = List::new(items).block(block);

        list = if self.menu.is_active() {
            list.highlight_style(highlight_style())
        } else {
            list
        };

        frame.render_stateful_widget(list, layout[0], self.menu.state());

        let mut block = Block::default().borders(Borders::all());
        block = if self.menu.is_active() {
            block
        } else {
            block.border_style(highlight_style())
        };

        let paragraph = Paragraph::new(self.input.value()).block(block);

        frame.render_widget(paragraph, layout[1]);

        if self.input.is_active() {
            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                layout[1].x + self.input.character_index() as u16 + 1,
                // Move one line down, from the border to the input line
                layout[1].y + 1,
            ));
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::HighlightNext => {
                if self.menu.is_active() {
                    self.menu.select_next();
                }
            }
            Message::HighlightPrevious => {
                if self.menu.is_active() {
                    self.menu.select_previous();
                }
            }
            Message::EnterChar(c) => {
                if self.input.is_active() {
                    self.input.enter_char(c);
                }
            }
            Message::DeleteChar => {
                if self.input.is_active() {
                    self.input.delete_char();
                }
            }
            Message::DeleteAllChars => {
                if self.input.is_active() {
                    self.input.delete_all_chars();
                }
            }
            Message::MoveCusorLeft => {
                if self.input.is_active() {
                    self.input.move_cursor_left();
                }
            }
            Message::MoveCusorRight => {
                if self.input.is_active() {
                    self.input.move_cursor_right();
                }
            }
            Message::Exit => self.status = Some(Status::Exit),
            Message::Back => self.status = Some(Status::Back),
            Message::HighlightMenu => {
                self.menu.activate();
                self.input.deactivate();
            }
            Message::HighlightContent => {
                self.menu.deactivate();
                self.input.activate();
            }
            Message::Sumbit => {
                if self.menu.is_active() {
                    if let Some(item) = self.menu.item() {
                        match item {
                            MenuItem::Exit => self.status = Some(Status::Exit),
                            MenuItem::Back => self.status = Some(Status::Back),
                            _ => {}
                        }
                    }
                }

                if self.input.is_active() {
                    self.status = Some(Status::Submit);
                }
            }
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
        KeyCode::Left => match key_event.modifiers {
            KeyModifiers::ALT => Message::HighlightMenu,
            _ => Message::MoveCusorLeft,
        },
        KeyCode::Right => match key_event.modifiers {
            KeyModifiers::ALT => Message::HighlightContent,
            _ => Message::MoveCusorRight,
        },
        KeyCode::Up => Message::HighlightPrevious,
        KeyCode::Down => Message::HighlightNext,
        _ => return None,
    };

    Some(message)
}
