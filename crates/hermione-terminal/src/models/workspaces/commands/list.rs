use crate::{
    breadcrumbs::Breadcrumbs,
    layouts, parameters, presenters,
    routes::{self, Route},
    smart_input::{NewSmartInputParameters, SmartInput, Value},
    widgets, Error, Message, Result,
};
use hermione_tui::app::{self, EventHandler};
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    workspace: presenters::workspace::Presenter,
    commands: Vec<presenters::command::Presenter>,
    redirect: Option<Route>,
    commands_state: widgets::list::State,
    powershell_settings: PowerShellSettings,
    page_number: u32,
    page_size: u32,
    smart_input: SmartInput,
    search_query: String,
    is_running: bool,
}

pub struct ModelParameters {
    pub commands: Vec<presenters::command::Presenter>,
    pub page_number: u32,
    pub page_size: u32,
    pub powershell_no_exit: bool,
    pub search_query: String,
    pub workspace: presenters::workspace::Presenter,
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

impl app::Model for Model {
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
            Message::Action => self.execute_command(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit()?,
            Message::ToggleFocus => self.toggle_focus(),
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());
        let [list_area, input_area] = layouts::search_list::Layout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());
        let list = widgets::list::Widget::new(&self.commands).block(block);

        frame.render_stateful_widget(list, list_area, &mut self.commands_state);
        self.smart_input.render(frame, input_area);

        let paragraph = Paragraph::new(self.breadcrumbs());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl Model {
    fn toggle_focus(&mut self) {
        self.smart_input.toggle_input();
    }

    fn cancel(&mut self) {
        self.smart_input.reset_input();

        if !self.search_query.is_empty() {
            self.set_redirect(
                parameters::workspaces::commands::list::Parameters {
                    workspace_id: self.workspace.id.clone(),
                    search_query: "".into(),
                    page_number: 0,
                    page_size: self.page_size,
                    powershell_no_exit: self.powershell_settings.no_exit,
                }
                .into(),
            );
        }
    }

    fn breadcrumbs(&self) -> Breadcrumbs {
        let breadcrumbs = Breadcrumbs::default()
            .add_segment("List workspaces")
            .add_segment(&self.workspace.name)
            .add_segment(format!("List commands ({})", self.page_number));

        if let Some(command) = self.command() {
            breadcrumbs.add_segment(&command.name)
        } else {
            breadcrumbs
        }
    }

    fn execute_command(&mut self) {
        let Some(index) = self.commands_state.selected() else {
            return;
        };

        let command = self.commands.remove(index);

        let route = Route::Powershell(routes::powershell::Route::ExecuteCommand(
            parameters::powershell::execute_command::Parameters {
                command_id: command.id.clone(),
                workspace_id: command.workspace_id.clone(),
                powershell_no_exit: self.powershell_settings.no_exit,
            },
        ));

        self.redirect = Some(route);

        self.commands.insert(0, command);
        self.commands_state.select_first();
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    fn command(&self) -> Option<&presenters::command::Presenter> {
        self.commands_state
            .selected()
            .and_then(|index| self.commands.get(index))
    }

    fn copy_to_clipboard_parameters(
        &self,
    ) -> Option<parameters::powershell::copy_to_clipboard::Parameters> {
        self.command().map(
            |command| parameters::powershell::copy_to_clipboard::Parameters {
                workspace_id: self.workspace.id.clone(),
                command_id: command.id.clone(),
            },
        )
    }

    fn start_windows_terminal_parameters(
        &self,
    ) -> parameters::powershell::start_windows_terminal::Parameters {
        parameters::powershell::start_windows_terminal::Parameters {
            working_directory: self.workspace.location.clone(),
        }
    }

    fn copy_to_clipboard(&mut self) {
        self.redirect = self.copy_to_clipboard_parameters().map(|parameters| {
            Route::Powershell(routes::powershell::Route::CopyToClipboard(parameters))
        });

        self.smart_input.reset_input();
    }

    fn start_windows_terminal(&mut self) {
        self.redirect = Some(Route::Powershell(
            routes::powershell::Route::StartWindowsTerminal(
                self.start_windows_terminal_parameters(),
            ),
        ));

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

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters {
            commands,
            page_number,
            page_size,
            powershell_no_exit,
            search_query,
            workspace,
        } = parameters;

        let mut commands_state = widgets::list::State::default();

        if !commands.is_empty() {
            commands_state.select_first();
        }

        let mut model = Self {
            commands_state,
            commands,
            page_number,
            page_size,
            powershell_settings: PowerShellSettings {
                no_exit: powershell_no_exit,
            },
            redirect: None,
            smart_input: smart_input(),
            workspace,
            search_query,
            is_running: true,
        };

        if !model.search_query.is_empty() {
            for c in model.search_query.chars() {
                model.smart_input.enter_char(c);
            }
        }

        Ok(model)
    }

    fn select_next(&mut self) {
        let Some(index) = self.commands_state.selected() else {
            return;
        };

        if index == self.commands.len() - 1 {
            if self.commands.len() < self.page_size as usize {
                return;
            }

            self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                routes::workspaces::commands::Route::List(
                    parameters::workspaces::commands::list::Parameters {
                        search_query: self.search_query.clone(),
                        workspace_id: self.workspace.id.clone(),
                        page_number: self.page_number + 1,
                        page_size: self.page_size,
                        powershell_no_exit: self.powershell_settings.no_exit,
                    },
                ),
            )));

            return;
        }

        self.commands_state.select_next();
    }

    fn select_previous(&mut self) {
        let Some(index) = self.commands_state.selected() else {
            if self.page_number != 0 {
                self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                    routes::workspaces::commands::Route::List(
                        parameters::workspaces::commands::list::Parameters {
                            search_query: self.search_query.clone(),
                            workspace_id: self.workspace.id.clone(),
                            page_number: self.page_number - 1,
                            page_size: self.page_size,
                            powershell_no_exit: self.powershell_settings.no_exit,
                        },
                    ),
                )));
            }

            return;
        };

        if index == 0 {
            if self.page_number == 0 {
                return;
            }

            self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                routes::workspaces::commands::Route::List(
                    parameters::workspaces::commands::list::Parameters {
                        search_query: self.search_query.clone(),
                        workspace_id: self.workspace.id.clone(),
                        page_number: self.page_number - 1,
                        page_size: self.page_size,
                        powershell_no_exit: self.powershell_settings.no_exit,
                    },
                ),
            )));

            return;
        }

        self.commands_state.select_previous();
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn new_command_parameters(&self) -> parameters::workspaces::commands::new::Parameters {
        parameters::workspaces::commands::new::Parameters {
            workspace_id: self.workspace.id.clone(),
        }
    }

    fn submit(&mut self) -> Result<()> {
        let Some(Value::Command(command)) = self.smart_input.value() else {
            self.smart_input.reset_input();

            return Ok(());
        };

        let action = Action::try_from(command)?;

        match action {
            Action::DeleteCommand => {
                if let Some(command) = self.command() {
                    self.set_redirect(
                        parameters::workspaces::commands::delete::Parameters {
                            workspace_id: self.workspace.id.clone(),
                            command_id: command.id.clone(),
                        }
                        .into(),
                    )
                }
            }
            Action::EditCommand => {
                if let Some(command) = self.command() {
                    self.set_redirect(
                        parameters::workspaces::commands::edit::Parameters {
                            workspace_id: self.workspace.id.clone(),
                            command_id: command.id.clone(),
                        }
                        .into(),
                    )
                }
            }
            Action::Exit => self.exit(),
            Action::ListWorkspaces => self.set_redirect(Route::Workspaces(
                routes::workspaces::Route::List(parameters::workspaces::list::Parameters {
                    page_number: 0,
                    page_size: self.page_size,
                    search_query: "".into(),
                }),
            )),
            Action::NewCommand => {
                self.set_redirect(self.new_command_parameters().into());
            }
            Action::CopyToClipboard => self.copy_to_clipboard(),
            Action::StartWindowsTerminal => self.start_windows_terminal(),
            Action::PowerShellSetNoExit => self.powershell_set_no_exit(),
            Action::PowerShellUnsetNoExit => self.powershell_unset_no_exit(),
        }

        Ok(())
    }

    fn enter_char(&mut self, c: char) {
        self.smart_input.enter_char(c);

        let Some(Value::Base(search_query)) = self.smart_input.value() else {
            return;
        };

        self.set_redirect(
            parameters::workspaces::commands::list::Parameters {
                search_query: search_query.into(),
                workspace_id: self.workspace.id.clone(),
                page_number: 0,
                page_size: self.page_size,
                powershell_no_exit: self.powershell_settings.no_exit,
            }
            .into(),
        );
    }

    fn delete_char(&mut self) {
        self.smart_input.delete_char();

        let Some(Value::Base(search_query)) = self.smart_input.value() else {
            return;
        };

        self.set_redirect(
            parameters::workspaces::commands::list::Parameters {
                search_query: search_query.into(),
                workspace_id: self.workspace.id.clone(),
                page_number: 0,
                page_size: self.page_size,
                powershell_no_exit: self.powershell_settings.no_exit,
            }
            .into(),
        );
    }

    fn delete_all_chars(&mut self) {
        self.smart_input.reset_input();

        let Some(Value::Base(search_query)) = self.smart_input.value() else {
            return;
        };

        self.set_redirect(
            parameters::workspaces::commands::list::Parameters {
                search_query: search_query.into(),
                workspace_id: self.workspace.id.clone(),
                page_number: 0,
                page_size: self.page_size,
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
    Exit,
    ListWorkspaces,
    NewCommand,
    StartWindowsTerminal,
    PowerShellSetNoExit,
    PowerShellUnsetNoExit,
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        let action = match action {
            Action::CopyToClipboard => "Copy to clipboard",
            Action::DeleteCommand => "Delete command",
            Action::EditCommand => "Edit command",
            Action::Exit => "Exit",
            Action::ListWorkspaces => "List workspaces",
            Action::NewCommand => "New command",
            Action::StartWindowsTerminal => "Start Windows Terminal",
            Action::PowerShellSetNoExit => "Set PowerShell -NoExit",
            Action::PowerShellUnsetNoExit => "Unset PowerShell -NoExit",
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
            "Exit" => Ok(Self::Exit),
            "List workspaces" => Ok(Self::ListWorkspaces),
            "New command" => Ok(Self::NewCommand),
            "Set PowerShell -NoExit" => Ok(Self::PowerShellSetNoExit),
            "Start Windows Terminal" => Ok(Self::StartWindowsTerminal),
            "Unset PowerShell -NoExit" => Ok(Self::PowerShellUnsetNoExit),
            _ => Err(anyhow::anyhow!("Unknown action: {}", value)),
        }
    }
}

fn smart_input() -> SmartInput {
    SmartInput::new(NewSmartInputParameters {
        commands: vec![
            Action::CopyToClipboard.into(),
            Action::DeleteCommand.into(),
            Action::EditCommand.into(),
            Action::Exit.into(),
            Action::ListWorkspaces.into(),
            Action::NewCommand.into(),
            Action::PowerShellSetNoExit.into(),
            Action::PowerShellUnsetNoExit.into(),
            Action::StartWindowsTerminal.into(),
        ],
    })
}
