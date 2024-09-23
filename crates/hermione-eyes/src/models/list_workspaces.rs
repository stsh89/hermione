use crate::{
    entities::Workspace,
    models::{
        handle_event, highlight_style,
        shared::{Menu, MenuItem},
        Message, Redirect,
    },
    router::Router,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct ListWorkspacesModel {
    workspaces: Vec<Workspace>,
    status: Option<Status>,
    workspaces_state: ListState,
    menu: Menu,
}

pub struct ListWorkspacesModelParameters {
    pub workspaces: Vec<Workspace>,
}

enum Status {
    Exit,
    CreateWorkspace,
}

impl ListWorkspacesModel {
    pub fn new(parameters: ListWorkspacesModelParameters) -> Self {
        let ListWorkspacesModelParameters { workspaces } = parameters;

        let mut model = Self {
            workspaces,
            status: None,
            workspaces_state: ListState::default(),
            menu: Menu::new(vec![MenuItem::CreateWorkspace, MenuItem::Exit]),
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        model
    }

    pub fn redirect(&self) -> Option<Redirect> {
        self.status.as_ref().map(|status| match status {
            Status::Exit => Redirect::Exit,
            Status::CreateWorkspace => Redirect::Route(Router::NewWorkspace),
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
        block = if !self.menu.is_active() {
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
        let mut list = List::new(items).block(block);

        list = if self.menu.is_active() {
            list
        } else {
            list.highlight_style(highlight_style())
        };

        frame.render_stateful_widget(list, layout[1], &mut self.workspaces_state);
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::CreateWorkspace => self.status = Some(Status::CreateWorkspace),
            Message::HighlightMenu => {
                self.menu.activate();
            }
            Message::HighlightContent => {
                self.menu.deactivate();
            }
            Message::HighlightNext => {
                if self.menu.is_active() {
                    self.menu.select_next();
                } else {
                    self.workspaces_state.select_next()
                }
            }
            Message::HighlightPrevious => {
                if self.menu.is_active() {
                    self.menu.select_previous();
                } else {
                    self.workspaces_state.select_previous()
                }
            }
            Message::Exit => self.status = Some(Status::Exit),
            Message::Sumbit => {
                if self.menu.is_active() {
                    if let Some(item) = self.menu.item() {
                        match item {
                            MenuItem::Exit => self.status = Some(Status::Exit),
                            MenuItem::CreateWorkspace => return Ok(Some(Message::CreateWorkspace)),
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
            _ => return None,
        },
        KeyCode::Right => match key_event.modifiers {
            KeyModifiers::ALT => Message::HighlightContent,
            _ => return None,
        },
        KeyCode::Char('c') => Message::CreateWorkspace,
        _ => return None,
    };

    Some(message)
}
