mod list_workspaces;

use crate::{router::Router, Result};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
use ratatui::{crossterm::event, Frame};

pub enum Model {
    ListWorkspaces(ListWorkspacesModel),
}

pub enum Message {
    HighlightNext,
    HighlightPrevious,
    Exit,
}

enum State {
    Running(Router),
    Exited(Option<Router>),
}

impl Model {
    pub fn route(&self) -> Option<Router> {
        match self {
            Model::ListWorkspaces(model) => model.route(),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        match self {
            Model::ListWorkspaces(model) => model.view(frame),
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.handle_event(),
        }
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.update(message),
        }
    }
}

fn handle_event<F>(f: F) -> Result<Option<Message>>
where
    F: Fn(event::KeyEvent) -> Option<Message>,
{
    if let event::Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            let message = f(key);

            return Ok(message);
        }
    }

    Ok(None)
}
