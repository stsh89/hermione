use crate::{
    layouts::{SearchListLayout, WideLayout},
    smart_input::{NewSmartInputParameters, SmartInput},
    themes::{Theme, Themed},
    widgets::{StatusBar, StatusBarWidget},
    CommandPresenter, CopyCommandToClipboardParams, DeleteCommandParams, EditCommandParams, Error,
    ExecuteCommandParams, ListWorkspaceCommandsParams, ListWorkspacesParams, Message,
    NewWorkspaceCommandParams, OpenWindowsTerminalParams, PowerShellRoute, Result, Route,
    WorkspacePresenter,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{
    widgets::{Block, Borders, List, ListState},
    Frame,
};
use std::num::NonZeroU32;

pub struct ListWorkspaceCommandsModel {
    workspace: WorkspacePresenter,
    commands: Vec<CommandPresenter>,
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

pub struct ListWorkspaceCommandsModelParameters {
    pub commands: Vec<CommandPresenter>,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub powershell_no_exit: bool,
    pub search_query: String,
    pub workspace: WorkspacePresenter,
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

impl Model for ListWorkspaceCommandsModel {
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

        let block = Block::default().borders(Borders::all()).themed(self.theme);
        let list = List::new(&self.commands).block(block).themed(self.theme);

        frame.render_stateful_widget(list, list_area, &mut self.commands_state);
        self.smart_input.render(frame, input_area);

        frame.render_widget(
            StatusBarWidget::new(&self.status_bar()).themed(self.theme),
            status_bar_area,
        );
    }
}

impl ListWorkspaceCommandsModel {
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
                    page_number: NonZeroU32::new(1),
                    page_size: Some(self.page_size),
                    powershell_no_exit: self.powershell_settings.no_exit,
                }
                .into(),
            );
        }
    }

    fn status_bar(&self) -> StatusBar {
        let mut builder = StatusBar::builder()
            .operation("List commands")
            .workspace(&self.workspace.name)
            .page(self.page_number);

        if let Some(command) = self.command() {
            builder = builder.selector(&command.name);
        }

        if self.powershell_settings.no_exit {
            builder = builder.pwsh("-NoExit");
        }

        if !self.search_query.is_empty() {
            builder = builder.search(&self.search_query);
        }

        builder.build()
    }

    fn execute_command(&mut self) {
        let Some(index) = self.commands_state.selected() else {
            return;
        };

        let command = self.commands.remove(index);

        let route = Route::Powershell(PowerShellRoute::ExecuteCommand(ExecuteCommandParams {
            command_id: command.id,
            workspace_id: command.workspace_id,
            powershell_no_exit: self.powershell_settings.no_exit,
        }));

        self.redirect = Some(route);

        self.commands.insert(0, command);
        self.commands_state.select_first();
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    fn command(&self) -> Option<&CommandPresenter> {
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
            working_directory: self.workspace.location.clone(),
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

    pub fn new(parameters: ListWorkspaceCommandsModelParameters) -> Result<Self> {
        let ListWorkspaceCommandsModelParameters {
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
            page_number: page_number.unwrap_or_else(|| NonZeroU32::new(1).unwrap()),
            page_size: page_size.unwrap_or_else(|| NonZeroU32::new(10).unwrap()),
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
                        page_number: NonZeroU32::new(1),
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
                page_number: NonZeroU32::new(1),
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
                page_number: NonZeroU32::new(1),
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
                page_number: NonZeroU32::new(1),
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
