use crate::{
    entities::Workspace,
    models::{handle_event, highlight_style, Menu, MenuItem, Message, State},
    router::Router,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct ListWorkspacesModel {
    workspaces: Vec<Workspace>,
    state: State,
    workspaces_state: ListState,
    menu: Menu,
}

pub struct ListWorkspacesModelParameters {
    pub workspaces: Vec<Workspace>,
}

impl ListWorkspacesModel {
    pub fn new(parameters: ListWorkspacesModelParameters) -> Self {
        let ListWorkspacesModelParameters { workspaces } = parameters;

        let mut model = Self {
            workspaces,
            state: State::Running(Router::ListWorkspaces),
            workspaces_state: ListState::default(),
            menu: Menu::new(vec![MenuItem::Exit, MenuItem::CreateWorkspace]),
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        model
    }

    pub fn route(&self) -> Option<&Router> {
        match &self.state {
            State::Running(route) => Some(route),
            State::Exited(route) => route.as_ref(),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
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
        block = if !self.menu.is_active {
            block.border_style(highlight_style())
        } else {
            block
        };

        if self.workspaces.is_empty() {
            let paragraph = Paragraph::new("Start by creating a workspace").block(block);
            frame.render_widget(paragraph, layout[1]);
            return;
        }

        let items: Vec<ListItem> = self.workspaces.iter().map(ListItem::from).collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());

        frame.render_stateful_widget(list, layout[1], &mut self.workspaces_state);
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::HighlightNext => {
                if self.menu.is_active {
                    self.menu.select_next();
                } else {
                    self.workspaces_state.select_next()
                }
            }
            Message::HighlightPrevious => {
                if self.menu.is_active {
                    self.menu.select_previous();
                } else {
                    self.workspaces_state.select_previous()
                }
            }
            Message::Exit => self.state = State::Exited(None),
            Message::ToggleFocus => self.menu.toggle_focus(),
            Message::Sumbit => {
                if self.menu.is_active {
                    if let Some(index) = self.menu.state.selected() {
                        match self.menu.items[index] {
                            MenuItem::Exit => self.state = State::Exited(None),
                            MenuItem::CreateWorkspace => {
                                self.state = State::Exited(Some(Router::NewWorkspace))
                            }
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
        KeyCode::Tab => Message::ToggleFocus,
        KeyCode::Enter => Message::Sumbit,
        _ => return None,
    };

    Some(message)
}
