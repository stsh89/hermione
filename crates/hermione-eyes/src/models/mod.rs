mod list_workspaces;
mod new_workspace;
mod shared;

use crate::{router::Router, Result};
pub use list_workspaces::{ListWorkspacesModel, ListWorkspacesModelParameters};
pub use new_workspace::NewWorkspaceModel;
use ratatui::{
    crossterm::event,
    style::{Style, Stylize},
    widgets::{ListItem, ListState},
    Frame,
};

pub enum Model {
    ListWorkspaces(ListWorkspacesModel),
    NewWorkspace(NewWorkspaceModel),
}

enum MenuItem {
    Back,
    CreateWorkspace,
    Exit,
}

struct Menu {
    items: Vec<MenuItem>,
    state: ListState,
    is_active: bool,
}

impl<'a> From<&MenuItem> for ListItem<'a> {
    fn from(menu_item: &MenuItem) -> Self {
        let name = match menu_item {
            MenuItem::Exit => "Exit",
            MenuItem::CreateWorkspace => "Create workspace",
            MenuItem::Back => "Back",
        };

        ListItem::new(name)
    }
}

impl Menu {
    fn new(items: Vec<MenuItem>) -> Self {
        let mut menu = Self {
            items,
            state: ListState::default(),
            is_active: false,
        };

        if !menu.items.is_empty() {
            menu.state.select_first();
        }

        menu
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn toggle_focus(&mut self) {
        self.is_active = !self.is_active;
    }
}

pub enum Message {
    Back,
    DeleteAllChars,
    DeleteChar,
    EnterChar(char),
    Exit,
    HighlightNext,
    HighlightPrevious,
    MoveCusorLeft,
    MoveCusorRight,
    Sumbit,
    ToggleFocus,
}

pub enum Redirect {
    Exit,
    Route(Router),
}

impl Model {
    pub fn redirect(&self) -> Option<Redirect> {
        match self {
            Model::ListWorkspaces(model) => model.redirect(),
            Model::NewWorkspace(model) => model.redirect(),
        }
    }

    pub fn view(&mut self, frame: &mut Frame) {
        match self {
            Model::ListWorkspaces(model) => model.view(frame),
            Model::NewWorkspace(model) => model.view(frame),
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.handle_event(),
            Model::NewWorkspace(model) => model.handle_event(),
        }
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match self {
            Model::ListWorkspaces(model) => model.update(message),
            Model::NewWorkspace(model) => model.update(message),
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
