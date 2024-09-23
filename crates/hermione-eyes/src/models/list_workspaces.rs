use crate::{
    entities::Workspace,
    models::{handle_event, Message, State},
    router::Router,
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Layout},
    widgets::{List, ListItem, ListState},
    Frame,
};

pub struct ListWorkspacesModel {
    workspaces: Vec<Workspace>,
    state: State,
    list_state: ListState,
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
            list_state: ListState::default(),
        };

        if !model.workspaces.is_empty() {
            model.list_state.select_first();
        }

        model
    }

    pub fn route(&self) -> Option<Router> {
        match self.state {
            State::Running(route) => Some(route),
            State::Exited(route) => route,
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .constraints(vec![Constraint::Percentage(100)])
            .split(frame.area());

        let items: Vec<ListItem> = self.workspaces.iter().map(ListItem::from).collect();

        let list = List::new(items);

        frame.render_stateful_widget(list, layout[0], &mut self.list_state);
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::HighlightNext => self.list_state.select_next(),
            Message::HighlightPrevious => self.list_state.select_previous(),
            Message::Exit => self.state = State::Exited(None),
        }

        Ok(None)
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Up => Message::HighlightNext,
        KeyCode::Down => Message::HighlightPrevious,
        KeyCode::Esc => Message::Exit,
        _ => return None,
    };

    Some(message)
}
