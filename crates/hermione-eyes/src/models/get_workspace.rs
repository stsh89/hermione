use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    entities::{Command, Workspace},
    models::{
        helpers::{CommandPalette, CommandPaletteParameters, Input, InputParameters},
        highlight_style, Message, Model,
    },
    router::{
        DeleteWorkspaceParameters, EditWorkspaceParameters, ExecuteCommandParameters,
        GetCommandParameters, GetWorkspaceParameters, ListWorkspacesParameters,
        NewCommandParameters, Router,
    },
    Result,
};

pub struct GetWorkspaceModel {
    workspace: Workspace,
    commands: Vec<Command>,
    redirect: Option<Router>,
    search: Input,
    commands_state: ListState,
    command_palette: CommandPalette,
    is_running: bool,
}

pub struct GetWorkspaceModelParameters {
    pub commands: Vec<Command>,
    pub workspace: Workspace,
    pub commands_search_query: String,
}

impl Model for GetWorkspaceModel {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Router> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ToggleCommandPalette => self.toggle_command_palette(),
            Message::Back => self.back(),
            Message::ExecuteCommand => self.execute_command(),
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
        let items: Vec<ListItem> = self.commands.iter().map(ListItem::from).collect();

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
    fn execute_command(&mut self) {
        let Some(command) = self
            .commands_state
            .selected()
            .and_then(|i| self.commands.get(i))
        else {
            return;
        };

        self.redirect = Some(Router::ExecuteCommand(ExecuteCommandParameters {
            command_id: command.id().to_string(),
            workspace_id: self.workspace.id().to_string(),
        }));
    }

    fn handle_command_palette_action(&mut self) {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let Some(action) = self.command_palette.action() else {
            return;
        };

        match action {
            CPA::DeleteWorkspace => {
                self.redirect = Some(Router::DeleteWorkspace(DeleteWorkspaceParameters {
                    id: self.workspace.id().to_string(),
                }))
            }
            CPA::NewCommand => {
                self.redirect = Some(Router::NewCommand(NewCommandParameters {
                    workspace_id: self.workspace.id().to_string(),
                }))
            }
            CPA::ListWorkspaces => {
                self.redirect = Some(Router::ListWorkspaces(ListWorkspacesParameters::default()))
            }
            CPA::EditWorkspace => {
                self.redirect = Some(Router::EditWorkspace(EditWorkspaceParameters {
                    id: self.workspace.id().to_string(),
                }))
            }
            _ => {}
        }
    }

    pub fn new(parameters: GetWorkspaceModelParameters) -> Result<Self> {
        use crate::models::helpers::CommandPaletteAction as CPA;

        let GetWorkspaceModelParameters {
            commands,
            workspace,
            commands_search_query,
        } = parameters;

        let mut commands_state = ListState::default();

        if !commands.is_empty() {
            commands_state.select_first();
        }

        let model = Self {
            is_running: true,
            commands,
            workspace,
            redirect: None,
            search: Input::new(InputParameters {
                value: commands_search_query,
                is_active: true,
            }),
            commands_state,
            command_palette: CommandPalette::new(CommandPaletteParameters {
                actions: vec![
                    CPA::DeleteWorkspace,
                    CPA::EditWorkspace,
                    CPA::ListWorkspaces,
                    CPA::NewCommand,
                ],
            })?,
        };

        Ok(model)
    }

    fn back(&mut self) {
        if self.command_palette.is_active() {
            self.command_palette.toggle();

            return;
        }

        self.is_running = false;
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
            .and_then(|i| self.commands.get(i))
        else {
            return;
        };

        self.redirect = Some(Router::GetCommand(GetCommandParameters {
            workspace_id: self.workspace.id().to_string(),
            command_id: command.id().to_string(),
        }));
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            id: self.workspace.id().to_string(),
        }));
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            id: self.workspace.id().to_string(),
        }));
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        self.redirect = Some(Router::GetWorkspace(GetWorkspaceParameters {
            commands_search_query: self.search_query(),
            id: self.workspace.id().to_string(),
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
