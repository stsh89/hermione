pub mod command_palette;

mod create_workspace;
mod list_workspaces;
mod new_workspace;
mod shared;

use crate::{router::Router, Result};
use command_palette::CommandPaletteModel;
use ratatui::{
    crossterm::event,
    style::{Style, Stylize},
    Frame,
};

pub use create_workspace::{CreateWorkspaceModel, CreateWorkspaceModelParameters};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
pub use new_workspace::NewWorkspaceModel;

pub enum Model {
    ListWorkspaces(ListWorkspacesModel),
    NewWorkspace(NewWorkspaceModel),
    CreateWorkspace(CreateWorkspaceModel),
    CommandPalette(CommandPaletteModel),
}

pub enum Message {
    ActivateCommandPalette,
    Back,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    Exit,
    HighlightContent,
    HighlightMenu,
    SelectNext,
    SelectPrevious,
    MoveCusorLeft,
    MoveCusorRight,
    Sumbit,
    ToggleForcus,
}

impl Model {
    pub fn handle_event(&self) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.handle_event(),
            Model::NewWorkspace(model) => model.handle_event(),
            Model::CreateWorkspace(model) => model.handle_event(),
            Model::CommandPalette(model) => model.handle_event(),
        }
    }

    pub fn is_running(&self) -> bool {
        match self {
            Model::ListWorkspaces(model) => model.is_running(),
            Model::NewWorkspace(model) => model.is_running(),
            Model::CreateWorkspace(model) => model.is_running(),
            Model::CommandPalette(model) => model.is_running(),
        }
    }

    pub fn redirect(&self) -> Option<&Router> {
        match self {
            Model::ListWorkspaces(model) => model.redirect(),
            Model::NewWorkspace(model) => model.redirect(),
            Model::CreateWorkspace(model) => model.redirect(),
            Model::CommandPalette(model) => model.redirect(),
        }
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.update(message),
            Model::NewWorkspace(model) => model.update(message),
            Model::CreateWorkspace(model) => model.update(message),
            Model::CommandPalette(model) => model.update(message),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        match self {
            Model::ListWorkspaces(model) => model.view(frame),
            Model::NewWorkspace(model) => model.view(frame),
            Model::CreateWorkspace(model) => model.view(frame),
            Model::CommandPalette(model) => model.view(frame),
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
    Style::default().on_light_blue()
}
