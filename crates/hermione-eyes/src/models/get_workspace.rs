use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    entities::Workspace,
    models::{
        command_palette::{DELETE_WORKSPACE, NEW_COMMAND, RENAME_WORKSPACE},
        handle_event, highlight_style,
        shared::{Input, InputParameters},
        Message,
    },
    router::{
        CommandPaletteParameters, GetCommandParameters, GetWorkspaceParameters,
        ListWorkspacesParameters, Router,
    },
    Result,
};

pub struct GetWorkspaceModel {
    workspace: Workspace,
    is_running: bool,
    redirect: Option<Router>,
    search: Input,
    commands_state: ListState,
}

pub struct GetWorkspaceModelParameters {
    pub workspace: Workspace,
    pub commands_search_query: String,
}

impl GetWorkspaceModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(parameters: GetWorkspaceModelParameters) -> Self {
        let GetWorkspaceModelParameters {
            workspace,
            commands_search_query,
        } = parameters;

        let mut model = Self {
            workspace,
            is_running: true,
            redirect: None,
            search: Input::new(InputParameters {
                value: commands_search_query,
                is_active: true,
            }),
            commands_state: ListState::default(),
        };

        if !model.workspace.commands.is_empty() {
            model.commands_state.select_first();
        }

        model
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        handle_event(message)
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Back => self.redirect_to_list_workspaces(),
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

    fn redirect_to_list_workspaces(&mut self) {
        self.redirect = Some(Router::ListWorkspaces(ListWorkspacesParameters {
            search_query: String::new(),
        }))
    }

    fn submit(&mut self) {
        let Some(command) = self
            .commands_state
            .selected()
            .and_then(|i| self.workspace.commands.get(i))
        else {
            return;
        };

        self.redirect = Some(Router::GetCommand(GetCommandParameters {
            number: command.number,
        }));
    }

    fn select_next(&mut self) {
        self.commands_state.select_next();
    }

    fn select_previous(&mut self) {
        self.commands_state.select_previous();
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            number: self.workspace.number,
        }));
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            number: self.workspace.number,
        }));
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            number: self.workspace.number,
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
            actions: vec![NEW_COMMAND.to_string(), DELETE_WORKSPACE.to_string()],
        }))
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

        let paragraph = Paragraph::new(self.workspace.name.as_str()).alignment(Alignment::Center);
        frame.render_widget(paragraph, header);

        let block = Block::default().borders(Borders::all()).title("Search");
        let paragraph = Paragraph::new(self.search.value()).block(block);

        frame.render_widget(paragraph, search);
        frame.set_cursor_position(Position::new(
            search.x + self.search.character_index() as u16 + 1,
            search.y + 1,
        ));

        let block = Block::default().borders(Borders::all()).title("Commands");
        let items: Vec<ListItem> = self.workspace.commands.iter().map(ListItem::from).collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style());

        frame.render_stateful_widget(list, commands, &mut self.commands_state);
    }
}

fn message(key_event: KeyEvent) -> Option<Message> {
    let message = match key_event.code {
        KeyCode::Up => Message::SelectPrevious,
        KeyCode::Down => Message::SelectNext,
        KeyCode::Esc => Message::Back,
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
