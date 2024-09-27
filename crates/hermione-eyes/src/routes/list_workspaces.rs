use crate::{
    clients::memories,
    helpers::{
        Color, CommandPalette, CommandPaletteAction, CommandPaletteParameters, Input,
        InputParameters,
    },
    tea::{Hook, Message},
    router::{GetWorkspaceParameters, ListWorkspacesParameters, Router},
    types::{Result, Workspace},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct Handler<'a> {
    pub memories: &'a memories::Client,
}

pub struct Model {
    is_running: bool,
    redirect: Option<Router>,
    search: Input,
    workspaces_state: ListState,
    workspaces: Vec<Workspace>,
    command_palette: CommandPalette,
}

pub struct ModelParameters {
    pub workspaces: Vec<Workspace>,
    pub search_query: String,
}

impl<'a> Handler<'a> {
    pub fn handle(self, parameters: ListWorkspacesParameters) -> Result<Model> {
        let ListWorkspacesParameters { search_query } = parameters;
        let mut workspaces = self.memories.list_workspaces()?;
        let filter = search_query.to_lowercase();

        if !filter.is_empty() {
            workspaces.retain(|w| w.name.to_lowercase().contains(&filter));
        }

        Model::new(ModelParameters {
            workspaces,
            search_query,
        })
    }
}

impl Hook for Model {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Router> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ToggleCommandPalette => self.toggle_command_palette(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Back => self.back(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit(),
            _ => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [header, search, commands] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Min(3),
            ])
            .areas(frame.area());

        let paragraph = Paragraph::new("List workspaces").alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Search");
        let paragraph = Paragraph::new(self.search.value()).block(block);

        frame.render_widget(paragraph, search);
        frame.set_cursor_position(Position::new(
            search.x + self.search.character_index() as u16 + 1,
            search.y + 1,
        ));

        let block = Block::default().borders(Borders::all()).title("Workspaces");
        let items: Vec<ListItem> = self.workspaces.iter().map(ListItem::from).collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Color::default().highlight());

        frame.render_stateful_widget(list, commands, &mut self.workspaces_state);

        if self.command_palette.is_active() {
            self.command_palette.render(frame, frame.area());
        }
    }
}

impl Model {
    fn back(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.toggle();

            return;
        }

        self.is_running = false;
    }

    fn handle_command_palette_action(&mut self) {
        use CommandPaletteAction as CPA;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        if let CPA::NewWorkspace = action {
            self.redirect = Some(Router::NewWorkspace)
        }
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters {
            workspaces,
            search_query,
        } = parameters;

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: ListState::default(),
            search: Input::new(InputParameters {
                value: search_query,
                is_active: true,
            }),
            is_running: true,
            command_palette: CommandPalette::new(CommandPaletteParameters {
                actions: vec![CommandPaletteAction::NewWorkspace],
            })?,
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        Ok(model)
    }

    fn submit(&mut self) {
        if self.command_palette.is_active() {
            self.handle_command_palette_action();

            return;
        }

        let maybe_workspace = self
            .workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i));

        let Some(workspace) = maybe_workspace else {
            return;
        };

        let route = Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: String::new(),
            id: workspace.id().to_string(),
        });

        self.redirect = Some(route);
    }

    fn select_next(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_next();
        } else {
            self.workspaces_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_previous();
        } else {
            self.workspaces_state.select_previous();
        }
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        self.redirect = Some(Router::ListWorkspaces(ListWorkspacesParameters {
            search_query: self.search_query(),
        }));
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        self.redirect = Some(Router::ListWorkspaces(ListWorkspacesParameters {
            search_query: self.search_query(),
        }));
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        self.redirect = Some(Router::ListWorkspaces(ListWorkspacesParameters {
            search_query: self.search_query(),
        }));
    }

    fn move_cursor_left(&mut self) {
        self.search.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.search.move_cursor_right();
    }

    fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
    }
}
