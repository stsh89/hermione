use crate::{
    entities::Workspace,
    models::{
        handle_event, highlight_style,
        shared::Input,
        shared::{Menu, MenuItem},
        Message, Redirect,
    },
    router::Router,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct ListWorkspacesModel {
    workspaces: Vec<Workspace>,
    workspaces_state: ListState,
    menu: Menu,
    search: Input,
    route: Router,
    is_running: bool,
    redirect: Option<Router>,
}

pub struct ListWorkspacesModelParameters {
    pub workspaces: Vec<Workspace>,
    pub route: Router,
}

impl ListWorkspacesModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(parameters: ListWorkspacesModelParameters) -> Self {
        let ListWorkspacesModelParameters { route, workspaces } = parameters;

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: ListState::default(),
            menu: Menu::new(vec![MenuItem::CreateWorkspace, MenuItem::Exit]),
            search: Input::active(),
            route,
            is_running: true,
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        model
    }

    pub fn route(&self) -> &Router {
        &self.route
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let [menu, content] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .areas(frame.area());

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

        frame.render_stateful_widget(list, menu, self.menu.state());

        let mut block = Block::default().borders(Borders::all());
        block = if !self.menu.is_active() {
            block.border_style(highlight_style())
        } else {
            block
        };

        if self.workspaces.is_empty() {
            let paragraph = Paragraph::new("Start by creating a workspace").block(block);
            frame.render_widget(paragraph, content);
            return;
        }

        let [search, search_items] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Max(3), Constraint::Min(3)])
            .areas(content);

        let mut block = Block::default().borders(Borders::all());
        block = if self.search.is_active() {
            block.border_style(highlight_style())
        } else {
            block
        };

        let paragraph = Paragraph::new(self.search.value()).block(block);
        frame.render_widget(paragraph, search);

        if self.search.is_active() {
            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                search.x + self.search.character_index() as u16 + 1,
                // Move one line down, from the border to the input line
                search.y + 1,
            ));
        }

        let mut block = Block::default().borders(Borders::all());
        block = if self.search.is_active() {
            block
        } else {
            block.border_style(highlight_style())
        };

        let search_query = self.search.value().to_lowercase();
        let items: Vec<ListItem> = if self.workspaces.is_empty() {
            self.workspaces.iter().map(ListItem::from).collect()
        } else {
            self.workspaces
                .iter()
                .filter(|workspace| workspace.name.to_lowercase().contains(&search_query))
                .map(ListItem::from)
                .collect()
        };

        let mut list = List::new(items).block(block);

        list = if self.search.is_active() {
            list
        } else {
            list.highlight_style(highlight_style())
        };

        frame.render_stateful_widget(list, search_items, &mut self.workspaces_state);
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::HighlightMenu => {
                self.menu.activate();
                self.search.deactivate();
            }
            Message::HighlightContent => {
                self.menu.deactivate();
            }
            Message::HighlightNext => {
                if self.menu.is_active() {
                    self.menu.select_next();
                }

                if self.search.is_active() {
                    self.search.deactivate();
                } else {
                    self.workspaces_state.select_next()
                }
            }
            Message::HighlightPrevious => {
                if self.menu.is_active() {
                    self.menu.select_previous();
                }

                if self.search.is_active() {
                    self.search.deactivate();
                } else {
                    self.workspaces_state.select_previous()
                }
            }
            Message::Exit => self.is_running = false,
            Message::ToggleForcus => {
                if self.search.is_active() {
                    self.search.deactivate();
                } else {
                    self.search.activate();
                }
            }
            Message::EnterChar(c) => {
                if !self.menu.is_active() {
                    self.workspaces_state.select_first();
                    self.search.activate();
                    self.search.enter_char(c);
                }
            }
            Message::DeleteChar => {
                if !self.menu.is_active() {
                    self.workspaces_state.select_first();
                    self.search.activate();
                    self.search.delete_char();
                }
            }
            Message::DeleteAllChars => {
                if !self.menu.is_active() {
                    self.workspaces_state.select_first();
                    self.search.activate();
                    self.search.delete_all_chars();
                }
            }
            Message::MoveCusorLeft => {
                if !self.menu.is_active() {
                    self.search.activate();
                    self.search.move_cursor_left();
                }
            }
            Message::MoveCusorRight => {
                if !self.menu.is_active() {
                    self.search.activate();
                    self.search.move_cursor_right();
                }
            }
            Message::Sumbit => {
                if self.menu.is_active() {
                    if let Some(item) = self.menu.item() {
                        match item {
                            MenuItem::Exit => self.is_running = false,
                            MenuItem::CreateWorkspace => self.redirect = Some(Router::NewWorkspace),
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(None)
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Up => Message::HighlightPrevious,
        KeyCode::Down => Message::HighlightNext,
        KeyCode::Esc => Message::Exit,
        KeyCode::Enter => Message::Sumbit,
        KeyCode::Left => match key_event.modifiers {
            KeyModifiers::ALT => Message::HighlightMenu,
            _ => Message::MoveCusorLeft,
        },
        KeyCode::Right => match key_event.modifiers {
            KeyModifiers::ALT => Message::HighlightContent,
            _ => Message::MoveCusorRight,
        },
        KeyCode::Backspace => match key_event.modifiers {
            KeyModifiers::CONTROL => Message::DeleteAllChars,
            _ => Message::DeleteChar,
        },
        KeyCode::Char(c) => Message::EnterChar(c),
        KeyCode::Tab => Message::ToggleForcus,
        _ => return None,
    };

    Some(message)
}
