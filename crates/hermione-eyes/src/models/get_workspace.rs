use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    entities::Workspace,
    models::{
        helpers::{CommandPalette, CommandPaletteParameters, Input, InputParameters},
        highlight_style, Message, Model,
    },
    router::{GetCommandParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};

pub struct GetWorkspaceModel {
    workspace: Workspace,
    redirect: Option<Router>,
    search: Input,
    commands_state: ListState,
    command_palette: CommandPalette,
}

pub struct GetWorkspaceModelParameters {
    pub workspace: Workspace,
    pub commands_search_query: String,
}

impl Model for GetWorkspaceModel {
    fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ToggleCommandPalette => self.toggle_command_palette(),
            Message::Back => self.back(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
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

        if self.command_palette.is_active() {
            self.command_palette.render(frame, frame.area());
        }
    }
}

impl GetWorkspaceModel {
    fn handle_command_palette_action(&mut self) {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        match action {
            CPA::DeleteWorkspace => self.redirect = Some(Router::DeleteWorkspace),
            CPA::NewCommand => self.redirect = Some(Router::NewCommand),
            _ => {}
        }
    }

    pub fn new(parameters: GetWorkspaceModelParameters) -> Result<Self> {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let GetWorkspaceModelParameters {
            workspace,
            commands_search_query,
        } = parameters;

        let mut commands_state = ListState::default();

        if !workspace.commands.is_empty() {
            commands_state.select_first();
        }

        let model = Self {
            workspace,
            redirect: None,
            search: Input::new(InputParameters {
                value: commands_search_query,
                is_active: true,
            }),
            commands_state,
            command_palette: CommandPalette::new(CommandPaletteParameters {
                actions: vec![CPA::NewCommand, CPA::DeleteWorkspace],
            })?,
        };

        Ok(model)
    }

    fn back(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.toggle();

            return;
        }

        let route = Router::ListWorkspaces(ListWorkspacesParameters {
            search_query: String::new(),
        });

        self.redirect = Some(route)
    }

    fn select_next(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_next();
        } else {
            self.commands_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.select_previous();
        } else {
            self.commands_state.select_previous();
        }
    }

    fn submit(&mut self) {
        if self.command_palette.is_active() {
            self.handle_command_palette_action();

            return;
        }

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

    fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
    }
}
