mod list_workspaces;
mod new_workspace;
mod shared;

use crate::{entities::Workspace, router::Router, Result};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
pub use new_workspace::NewWorkspaceModel;
use ratatui::{
    crossterm::event,
    style::{Style, Stylize},
    Frame,
};

pub enum Model {
    ListWorkspaces(ListWorkspacesModel),
    NewWorkspace(NewWorkspaceModel),
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
}

pub enum Redirect {
    Exit,
    Route(Router),
}

impl Model {
    pub fn handle_event(&self) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.handle_event(),
            Model::NewWorkspace(model) => model.handle_event(),
        }
    }

    pub fn list_workspaces(workspaces: Vec<Workspace>) -> Self {
        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters { workspaces });

        Model::ListWorkspaces(model)
    }

    pub fn new_workspace() -> Self {
        let model = NewWorkspaceModel::new();

        Model::NewWorkspace(model)
    }

    pub fn is_list_workspaces(&self) -> bool {
        matches!(self, Model::ListWorkspaces(_))
    }

    pub fn is_new_workspace(&self) -> bool {
        matches!(self, Model::NewWorkspace(_))
    }

    pub fn redirect(&self) -> Option<Redirect> {
        match self {
            Model::ListWorkspaces(model) => model.redirect(),
            Model::NewWorkspace(model) => model.redirect(),
        }
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.update(message),
            Model::NewWorkspace(model) => model.update(message),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        match self {
            Model::ListWorkspaces(model) => model.view(frame),
            Model::NewWorkspace(model) => model.view(frame),
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
