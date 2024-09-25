use crate::{
    entities::Workspace,
    models::{
        command_palette::NEW_WORKSPACE,
        helpers::{Input, InputParameters},
        highlight_style, Message, Model,
    },
    router::{CommandPaletteParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub struct ListWorkspacesModel {
    is_running: bool,
    redirect: Option<Router>,
    search: Input,
    workspaces_state: ListState,
    workspaces: Vec<Workspace>,
}

pub struct ListWorkspacesModelParameters {
    pub workspaces: Vec<Workspace>,
    pub search_query: String,
}

impl Model for ListWorkspacesModel {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ActivateCommandPalette => self.activate_command_palette(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Back => self.exit(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Sumbit => self.submit(),
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
            .highlight_style(highlight_style());

        frame.render_stateful_widget(list, commands, &mut self.workspaces_state);
    }
}

impl ListWorkspacesModel {
    fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn new(parameters: ListWorkspacesModelParameters) -> Self {
        let ListWorkspacesModelParameters {
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
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        model
    }

    fn submit(&mut self) {
        let maybe_workspace = self
            .workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i));

        let Some(workspace) = maybe_workspace else {
            return;
        };

        let route = Router::GetWorkspace(GetWorkspaceParameters {
            number: workspace.number,
            commands_search_query: String::new(),
        });

        self.redirect = Some(route);
    }

    fn select_next(&mut self) {
        self.workspaces_state.select_next();
    }

    fn select_previous(&mut self) {
        self.workspaces_state.select_previous();
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

    fn activate_command_palette(&mut self) {
        let route = Router::CommandPalette(CommandPaletteParameters {
            actions: vec![NEW_WORKSPACE.to_string()],
        });

        self.redirect = Some(route)
    }
}
