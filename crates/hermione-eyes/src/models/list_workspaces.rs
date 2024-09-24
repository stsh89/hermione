use crate::{
    entities::Workspace,
    models::{
        command_palette::NEW_WORKSPACE,
        handle_event, highlight_style,
        shared::{Input, InputParameters},
        Message,
    },
    router::{CommandPaletteParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
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

impl ListWorkspacesModel {
    pub fn is_running(&self) -> bool {
        self.is_running
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

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn view(&mut self, frame: &mut Frame) {
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

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Exit => self.is_running = false,
            Message::EnterChar(c) => self.enter_char(c),
            Message::DeleteChar => self.delete_char(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::Sumbit => self.submit(),
            Message::ActivateCommandPalette => self.redirect_to_command_palette(),
            _ => {}
        }

        Ok(None)
    }

    fn submit(&mut self) {
        let Some(workspace) = self
            .workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i))
        else {
            return;
        };

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            number: workspace.number,
            commands_search_query: String::new(),
        }));
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

    fn redirect_to_command_palette(&mut self) {
        self.redirect = Some(Router::CommandPalette(CommandPaletteParameters {
            actions: vec![NEW_WORKSPACE.to_string()],
        }))
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Up => Message::SelectPrevious,
        KeyCode::Down => Message::SelectNext,
        KeyCode::Esc => Message::Exit,
        KeyCode::Enter => Message::Sumbit,
        KeyCode::Left => Message::MoveCusorLeft,
        KeyCode::Right => Message::MoveCusorRight,
        KeyCode::Backspace => match key_event.modifiers {
            KeyModifiers::CONTROL => Message::DeleteAllChars,
            _ => Message::DeleteChar,
        },
        KeyCode::Char(c) => match key_event.modifiers {
            KeyModifiers::CONTROL => match c {
                'k' => Message::ActivateCommandPalette,
                _ => return None,
            },
            _ => Message::EnterChar(c),
        },
        _ => return None,
    };

    Some(message)
}
