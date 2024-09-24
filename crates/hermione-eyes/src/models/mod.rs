mod create_workspace;
mod list_workspaces;
mod new_workspace;
mod shared;

use crate::{entities::Workspace, router::Router, Result};
use ratatui::{
    crossterm::event,
    style::{Style, Stylize},
    Frame,
};

pub use create_workspace::{CreateWorkspaceModel, CreateWorkspaceModelParameters};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
pub use new_workspace::NewWorkspaceModel;
pub use new_workspace::NewWorkspaceModelParameters;

pub enum Model {
    ListWorkspaces(ListWorkspacesModel),
    NewWorkspace(NewWorkspaceModel),
    CreateWorkspace(CreateWorkspaceModel),
}

pub enum Message {
    Back,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    Exit,
    HighlightContent,
    HighlightMenu,
    HighlightNext,
    HighlightPrevious,
    MoveCusorLeft,
    MoveCusorRight,
    Sumbit,
    ToggleForcus,
}

pub struct Redirect {
    route: Router,
}

impl Model {
    pub fn handle_event(&self) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.handle_event(),
            Model::NewWorkspace(model) => model.handle_event(),
            Model::CreateWorkspace(model) => model.handle_event(),
        }
    }

    pub fn is_running(&self) -> bool {
        match self {
            Model::ListWorkspaces(model) => model.is_running(),
            Model::NewWorkspace(model) => model.is_running(),
            Model::CreateWorkspace(model) => model.is_running(),
        }
    }

    pub fn redirect(&self) -> Option<&Router> {
        match self {
            Model::ListWorkspaces(model) => model.redirect(),
            Model::NewWorkspace(model) => model.redirect(),
            Model::CreateWorkspace(model) => model.redirect(),
        }
    }

    pub fn route(&self) -> &Router {
        match self {
            Model::ListWorkspaces(model) => model.route(),
            Model::NewWorkspace(model) => model.route(),
            Model::CreateWorkspace(model) => model.route(),
        }
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.update(message),
            Model::NewWorkspace(model) => model.update(message),
            Model::CreateWorkspace(model) => model.update(message),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        match self {
            Model::ListWorkspaces(model) => model.view(frame),
            Model::NewWorkspace(model) => model.view(frame),
            Model::CreateWorkspace(model) => model.view(frame),
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

fn highlight_style() -> Style {
    Style::default().green()
}
