use crate::{
    models::{
        handle_event, highlight_style, shared::Input, Menu, MenuItem, Message, Router, State,
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
    state: State,
    menu: Menu,
}

impl NewWorkspaceModel {
    pub fn new() -> Self {
        Self {
            input: Input::active(),
            state: State::Running(Router::NewWorkspace),
            menu: Menu::new(vec![MenuItem::Back, MenuItem::Exit]),
        }
    }

    pub fn route(&self) -> Option<&Router> {
        match &self.state {
            State::Running(route) => Some(route),
            State::Exited(route) => route.as_ref(),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(frame.area());

        let mut block = Block::default().borders(Borders::all());
        block = if self.menu.is_active {
            block.border_style(highlight_style())
        } else {
            block
        };
        let items: Vec<ListItem> = self.menu.items.iter().map(ListItem::from).collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());

        frame.render_stateful_widget(list, layout[0], &mut self.menu.state);

        let mut block = Block::default().borders(Borders::all());
        block = if self.menu.is_active {
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
                if self.menu.is_active {
                    self.menu.select_next();
                }
            }
            Message::HighlightPrevious => {
                if self.menu.is_active {
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
            Message::Exit => self.state = State::Exited(None),
            Message::Back => self.state = State::Exited(Some(Router::ListWorkspaces)),
            Message::ToggleFocus => {
                self.menu.toggle_focus();
                self.input.toggle_focus();
            }
            Message::Sumbit => {
                if self.menu.is_active {
                    if let Some(index) = self.menu.state.selected() {
                        match self.menu.items[index] {
                            MenuItem::Exit => self.state = State::Exited(None),
                            MenuItem::Back => {
                                self.state = State::Exited(Some(Router::ListWorkspaces))
                            }
                            _ => {}
                        }
                    }
                }

                if self.input.is_active() {
                    self.state =
                        State::Exited(Some(Router::CreateWorkspace(CreateWorkspaceParameters {
                            name: self.input.value().to_string(),
                        })));
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
        KeyCode::Tab => Message::ToggleFocus,
        KeyCode::Enter => Message::Sumbit,
        KeyCode::Left => Message::MoveCusorLeft,
        KeyCode::Right => Message::MoveCusorRight,
        KeyCode::Up => Message::HighlightPrevious,
        KeyCode::Down => Message::HighlightNext,
        _ => return None,
    };

    Some(message)
}
