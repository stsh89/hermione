use crate::{
    app::{
        helpers::{Input, InputParameters},
        EditWorkspaceParameters, GetWorkspaceParameters, Hook, Message, Router,
        UpdateWorkspaceParameters,
    },
    clients::memories,
    types::{Result, Workspace},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Model {
    workspace: Workspace,
    active_input: WorkspaceProperty,
    location: Input,
    name: Input,
    redirect: Option<Router>,
}

pub struct ModelParameters {
    pub workspace: Workspace,
}

enum WorkspaceProperty {
    Name,
    Location,
}

impl Hook for Model {
    fn redirect(&mut self) -> Option<Router> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Back => self.back(),
            Message::ToggleFocus => self.toggle_focus(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Submit => self.submit(),
            _ => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, name, location] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("Edit workspace").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Name");
        let paragraph = Paragraph::new(self.name.value()).block(block);

        frame.render_widget(paragraph, name);

        if self.name.is_active() {
            frame.set_cursor_position(Position::new(
                name.x + self.name.character_index() as u16 + 1,
                name.y + 1,
            ));
        }
        let block = Block::default().borders(Borders::all()).title("Location");
        let paragraph = Paragraph::new(self.location.value()).block(block);

        frame.render_widget(paragraph, location);

        if self.location.is_active() {
            frame.set_cursor_position(Position::new(
                location.x + self.location.character_index() as u16 + 1,
                location.y + 1,
            ));
        }
    }
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: EditWorkspaceParameters) -> Result<Model> {
        let EditWorkspaceParameters { id } = parameters;

        let workspace = self.memories.get_workspace(&id)?;

        let model = Model::new(ModelParameters { workspace });

        Ok(model)
    }
}

impl Model {
    fn back(&mut self) {
        let route = Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: String::new(),
            id: self.workspace.id.clone(),
        });

        self.redirect = Some(route);
    }

    fn delete_char(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.delete_char(),
            WorkspaceProperty::Location => self.location.delete_char(),
        }
    }

    fn delete_all_chars(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.delete_all_chars(),
            WorkspaceProperty::Location => self.location.delete_all_chars(),
        }
    }

    fn enter_char(&mut self, c: char) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.enter_char(c),
            WorkspaceProperty::Location => self.location.enter_char(c),
        }
    }

    fn move_cursor_left(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.move_cursor_left(),
            WorkspaceProperty::Location => self.location.move_cursor_left(),
        }
    }

    fn move_cursor_right(&mut self) {
        match self.active_input {
            WorkspaceProperty::Name => self.name.move_cursor_right(),
            WorkspaceProperty::Location => self.location.move_cursor_right(),
        }
    }

    pub fn new(parameters: ModelParameters) -> Self {
        let ModelParameters { workspace } = parameters;

        Self {
            name: Input::new(InputParameters {
                value: workspace.name.clone(),
                is_active: true,
            }),
            redirect: None,
            active_input: WorkspaceProperty::Name,
            location: Input::new(InputParameters {
                value: workspace.location.clone(),
                is_active: false,
            }),
            workspace,
        }
    }

    fn submit(&mut self) {
        let route = Router::UpdateWorkspace(UpdateWorkspaceParameters {
            name: self.name.value().to_string(),
            location: self.location.value().to_string(),
            id: self.workspace.id.clone(),
        });

        self.redirect = Some(route);
    }

    fn toggle_focus(&mut self) {
        self.active_input = match self.active_input {
            WorkspaceProperty::Name => WorkspaceProperty::Location,
            WorkspaceProperty::Location => WorkspaceProperty::Name,
        };

        match self.active_input {
            WorkspaceProperty::Name => {
                self.name.activate();
                self.location.deactivate();
            }
            WorkspaceProperty::Location => {
                self.location.activate();
                self.name.deactivate();
            }
        }
    }
}