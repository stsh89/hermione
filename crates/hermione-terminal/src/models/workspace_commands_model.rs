use super::{Command, Workspace};
use crate::{
    coordinator::{DEFAULT_PAGE_SIZE, FIRST_PAGE},
    layouts::{SearchListLayout, WideLayout},
    smart_input::{NewSmartInputParameters, SmartInput},
    themes::{Theme, HIGHLIGHT_SYMBOL},
    tui::{EventHandler, Model},
    CopyCommandToClipboardParams, DeleteCommandParams, EditCommandParams, Error,
    ExecuteCommandParams, ListWorkspaceCommandsParams, ListWorkspacesParams, Message,
    NewWorkspaceCommandParams, OpenWindowsTerminalParams, PowerShellRoute, Result, Route,
};
use ratatui::{
    layout::Rect,
    style::Stylize,
    text::ToText,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};
use std::num::NonZeroU32;

pub struct WorkspaceCommandsModel {
    workspace: Workspace,
    commands: Vec<Command>,
    redirect: Option<Route>,
    commands_state: ListState,
    powershell_settings: PowerShellSettings,
    page_number: NonZeroU32,
    page_size: NonZeroU32,
    smart_input: SmartInput,
    search_query: String,
    is_running: bool,
    theme: Theme,
}

pub struct WorkspaceCommandsModelParameters {
    pub commands: Vec<Command>,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub powershell_no_exit: bool,
    pub search_query: String,
    pub workspace: Workspace,
    pub theme: Theme,
}

struct PowerShellSettings {
    ///  Does not exit after running startup commands
    no_exit: bool,
}

impl PowerShellSettings {
    fn set_no_exit(&mut self) {
        self.no_exit = true;
    }

    fn unset_no_exit(&mut self) {
        self.no_exit = false;
    }
}

impl Model for WorkspaceCommandsModel {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::Cancel => self.cancel(),
            Message::ExecuteCommand => self.execute_command(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit()?,
            Message::Tab => self.toggle_focus(),
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let [list_area, input_area] = SearchListLayout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());
        let list = List::new(&self.commands)
            .block(block)
            .highlight_symbol(HIGHLIGHT_SYMBOL)
            .bg(self.theme.background_color)
            .fg(self.theme.foreground_color)
            .highlight_style(self.theme.highlight_color);

        frame.render_stateful_widget(list, list_area, &mut self.commands_state);
        self.smart_input.render(frame, input_area);
        self.render_status_bar(frame, status_bar_area);
    }
}

impl WorkspaceCommandsModel {
    fn toggle_focus(&mut self) {
        self.smart_input.autocomplete();
    }

    fn cancel(&mut self) {
        self.smart_input.reset_input();

        if !self.search_query.is_empty() {
            self.set_redirect(
                ListWorkspaceCommandsParams {
                    workspace_id: self.workspace.id,
                    search_query: "".into(),
                    page_number: None,
                    page_size: Some(self.page_size),
                    powershell_no_exit: self.powershell_settings.no_exit,
                }
                .into(),
            );
        }
    }

    fn execute_command(&mut self) {
        let Some(index) = self.commands_state.selected() else {
            return;
        };

        let command = self.commands.remove(index);

        let route = Route::Powershell(PowerShellRoute::ExecuteCommand(ExecuteCommandParams {
            command_id: command.id,
            powershell_no_exit: self.powershell_settings.no_exit,
        }));

        self.redirect = Some(route);

        self.commands.insert(0, command);
        self.commands_state.select_first();
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    fn command(&self) -> Option<&Command> {
        self.commands_state
            .selected()
            .and_then(|index| self.commands.get(index))
    }

    fn copy_to_clipboard_parameters(&self) -> Option<CopyCommandToClipboardParams> {
        self.command().map(|command| CopyCommandToClipboardParams {
            command_id: command.id,
        })
    }

    fn open_windows_terminal_parameters(&self) -> OpenWindowsTerminalParams {
        OpenWindowsTerminalParams {
            workspace_id: self.workspace.id,
        }
    }

    fn copy_to_clipboard(&mut self) {
        self.redirect = self
            .copy_to_clipboard_parameters()
            .map(|parameters| Route::Powershell(PowerShellRoute::CopyToClipboard(parameters)));

        self.smart_input.reset_input();
    }

    fn open_windows_terminal(&mut self) {
        self.redirect = Some(Route::Powershell(PowerShellRoute::OpenWindowsTerminal(
            self.open_windows_terminal_parameters(),
        )));

        self.smart_input.reset_input();
    }

    fn powershell_set_no_exit(&mut self) {
        self.powershell_settings.set_no_exit();
        self.smart_input.reset_input();
    }

    fn powershell_unset_no_exit(&mut self) {
        self.powershell_settings.unset_no_exit();
        self.smart_input.reset_input();
    }

    pub fn new(parameters: WorkspaceCommandsModelParameters) -> Result<Self> {
        let WorkspaceCommandsModelParameters {
            commands,
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace,
            theme,
        } = parameters;

        let mut commands_state = ListState::default();

        if !commands.is_empty() {
            commands_state.select_first();
        }

        let smart_input = SmartInput::new(NewSmartInputParameters {
            theme,
            commands: Action::all().into_iter().map(Into::into).collect(),
        });

        let mut model = Self {
            commands_state,
            commands,
            page_number: page_number.unwrap_or(FIRST_PAGE),
            page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
            powershell_settings: PowerShellSettings {
                no_exit: powershell_no_exit,
            },
            redirect: None,
            smart_input,
            workspace,
            search_query,
            is_running: true,
            theme,
        };

        if !model.search_query.is_empty() {
            for c in model.search_query.chars() {
                model.smart_input.enter_char(c);
            }
        }

        Ok(model)
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let mut value = serde_json::json!({
            "screen": "Commands",
            "page": self.page_number,
            "workspace": self.workspace.name,
        });

        if let Some(command) = self.command() {
            value["command_name"] = command.name.clone().into();
        }

        if let Some(search_query) = self.smart_input.search() {
            if !search_query.is_empty() {
                value["search"] = search_query.into();
            }
        }

        let paragraph = Paragraph::new(value.to_text())
            .bg(self.theme.status_bar_background_color)
            .fg(self.theme.status_bar_foreground_color);

        frame.render_widget(paragraph, area);
    }

    fn select_next(&mut self) {
        self.commands_state.select_next();
    }

    fn select_previous(&mut self) {
        self.commands_state.select_previous();
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn new_command_parameters(&self) -> NewWorkspaceCommandParams {
        NewWorkspaceCommandParams {
            workspace_id: self.workspace.id,
        }
    }

    fn submit(&mut self) -> Result<()> {
        if self.smart_input.search().is_some() {
            self.smart_input.reset_input();

            return Ok(());
        }

        let Some(command) = self.smart_input.command() else {
            return Ok(());
        };

        let action = Action::try_from(command)?;

        match action {
            Action::DeleteCommand => {
                if let Some(command) = self.command() {
                    self.set_redirect(
                        DeleteCommandParams {
                            workspace_id: self.workspace.id,
                            command_id: command.id,
                        }
                        .into(),
                    )
                }
            }
            Action::ExecuteCommand => self.execute_command(),
            Action::EditCommand => {
                if let Some(command) = self.command() {
                    self.set_redirect(
                        EditCommandParams {
                            command_id: command.id,
                        }
                        .into(),
                    )
                }
            }
            Action::Exit => self.exit(),
            Action::ListWorkspaces => {
                self.set_redirect(
                    ListWorkspacesParams {
                        page_number: None,
                        page_size: Some(self.page_size),
                        search_query: "".into(),
                    }
                    .into(),
                );
            }
            Action::NewCommand => {
                self.set_redirect(self.new_command_parameters().into());
            }
            Action::CopyToClipboard => self.copy_to_clipboard(),
            Action::OpenWindowsTerminal => self.open_windows_terminal(),
            Action::SetPowerShellNoExit => self.powershell_set_no_exit(),
            Action::UnsetPowerShellNoExit => self.powershell_unset_no_exit(),
        }

        self.smart_input.reset_input();

        Ok(())
    }

    fn enter_char(&mut self, c: char) {
        self.smart_input.enter_char(c);

        let Some(search_query) = self.smart_input.search() else {
            return;
        };

        self.set_redirect(
            ListWorkspaceCommandsParams {
                search_query: search_query.into(),
                workspace_id: self.workspace.id,
                page_number: None,
                page_size: Some(self.page_size),
                powershell_no_exit: self.powershell_settings.no_exit,
            }
            .into(),
        );
    }

    fn delete_char(&mut self) {
        self.smart_input.delete_char();

        let Some(search_query) = self.smart_input.search() else {
            return;
        };

        self.set_redirect(
            ListWorkspaceCommandsParams {
                search_query: search_query.into(),
                workspace_id: self.workspace.id,
                page_number: None,
                page_size: Some(self.page_size),
                powershell_no_exit: self.powershell_settings.no_exit,
            }
            .into(),
        );
    }

    fn delete_all_chars(&mut self) {
        self.smart_input.reset_input();

        let Some(search_query) = self.smart_input.search() else {
            return;
        };

        self.set_redirect(
            ListWorkspaceCommandsParams {
                search_query: search_query.into(),
                workspace_id: self.workspace.id,
                page_number: None,
                page_size: Some(self.page_size),
                powershell_no_exit: self.powershell_settings.no_exit,
            }
            .into(),
        );
    }

    fn move_cursor_left(&mut self) {
        self.smart_input.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.smart_input.move_cursor_right();
    }
}

enum Action {
    CopyToClipboard,
    DeleteCommand,
    EditCommand,
    ExecuteCommand,
    Exit,
    ListWorkspaces,
    NewCommand,
    OpenWindowsTerminal,
    SetPowerShellNoExit,
    UnsetPowerShellNoExit,
}

impl Action {
    fn all() -> Vec<Self> {
        vec![
            Self::CopyToClipboard,
            Self::DeleteCommand,
            Self::EditCommand,
            Self::ExecuteCommand,
            Self::Exit,
            Self::ListWorkspaces,
            Self::NewCommand,
            Self::OpenWindowsTerminal,
            Self::SetPowerShellNoExit,
            Self::UnsetPowerShellNoExit,
        ]
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        let action = match action {
            Action::CopyToClipboard => "Copy to clipboard",
            Action::DeleteCommand => "Delete command",
            Action::EditCommand => "Edit command",
            Action::ExecuteCommand => "Execute command",
            Action::Exit => "Exit",
            Action::ListWorkspaces => "List workspaces",
            Action::NewCommand => "New command",
            Action::OpenWindowsTerminal => "Open Windows Terminal",
            Action::SetPowerShellNoExit => "Set PowerShell -NoExit",
            Action::UnsetPowerShellNoExit => "Unset PowerShell -NoExit",
        };

        action.into()
    }
}

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "Copy to clipboard" => Ok(Self::CopyToClipboard),
            "Delete command" => Ok(Self::DeleteCommand),
            "Edit command" => Ok(Self::EditCommand),
            "Execute command" => Ok(Self::ExecuteCommand),
            "Exit" => Ok(Self::Exit),
            "List workspaces" => Ok(Self::ListWorkspaces),
            "New command" => Ok(Self::NewCommand),
            "Open Windows Terminal" => Ok(Self::OpenWindowsTerminal),
            "Set PowerShell -NoExit" => Ok(Self::SetPowerShellNoExit),
            "Unset PowerShell -NoExit" => Ok(Self::UnsetPowerShellNoExit),
            _ => Err(anyhow::anyhow!("Unknown action: {}", value)),
        }
    }
}
