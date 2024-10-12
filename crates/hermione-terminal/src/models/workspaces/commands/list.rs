use crate::{
    components,
    layouts::{self, Breadcrumbs},
    parameters, presenters,
    routes::{self, Route},
    tui::{self, Input},
    widgets, Message, Result,
};
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Model {
    workspace: presenters::workspace::Presenter,
    commands: Vec<presenters::command::Presenter>,
    redirect: Option<Route>,
    search: Input,
    commands_state: widgets::list::State,
    is_running: bool,
    powershell_settings: PowershellSettings,
    active_popup: Option<ActivePopup>,
    page_number: u32,
    page_size: u32,
}

pub struct ModelParameters {
    pub commands: Vec<presenters::command::Presenter>,
    pub workspace: presenters::workspace::Presenter,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

struct PowershellSettings {
    ///  Does not exit after running startup commands
    no_exit: bool,
}

impl PowershellSettings {
    fn set_no_exit(&mut self) {
        self.no_exit = true;
    }

    fn unset_no_exit(&mut self) {
        self.no_exit = false;
    }
}

enum ActivePopup {
    CommandPalette(components::command_palette::Component),
    ExitConfirmation(components::confirmation::Component),
}

impl ActivePopup {
    fn exit_confirmation() -> Self {
        let confirmation = components::confirmation::Component::new(
            components::confirmation::ComponentParameters {
                message: "Press \"Enter\" to exit...".into(),
            },
        );

        Self::ExitConfirmation(confirmation)
    }

    fn command_palette() -> Result<Self> {
        use components::command_palette::Action;

        let command_palette = components::command_palette::Component::new(
            components::command_palette::ComponentParameters {
                actions: vec![
                    Action::CopyToClipboard,
                    Action::DeleteWorkspace,
                    Action::EditWorkspace,
                    Action::ListWorkspaces,
                    Action::NewCommand,
                    Action::SetPowershellNoExit,
                    Action::StartWindowsTerminal,
                    Action::UnsetPowerShellNoExit,
                ],
            },
        )?;

        Ok(Self::CommandPalette(command_palette))
    }
}

impl tui::Model for Model {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        tui::EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        match message {
            Message::ActivateCommandPalette => self.activate_command_palette()?,
            Message::Back => self.back(),
            Message::Action => self.execute_command(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit(),
            Message::ToggleFocus => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::full_width::Layout::new().areas(frame.area());
        let [search_area, list_area] = layouts::search_list::Layout::new().areas(main_area);

        let block = Block::default().borders(Borders::ALL).title("Search");
        let paragraph = Paragraph::new(self.search.value()).block(block);
        self.search.render(frame, search_area, paragraph);

        let block = Block::default().borders(Borders::all());
        let list = widgets::list::Widget::new(&self.commands).block(block);

        frame.render_stateful_widget(list, list_area, &mut self.commands_state);

        let paragraph = Paragraph::new(self.breadcrumbs());
        frame.render_widget(paragraph, status_bar_area);

        let Some(popup) = &mut self.active_popup else {
            return;
        };

        match popup {
            ActivePopup::CommandPalette(popup) => popup.render(frame, frame.area()),
            ActivePopup::ExitConfirmation(popup) => popup.render(frame, frame.area()),
        }
    }
}

impl Model {
    fn activate_command_palette(&mut self) -> Result<()> {
        self.active_popup = Some(ActivePopup::command_palette()?);

        Ok(())
    }

    fn back(&mut self) {
        if self.active_popup.is_some() {
            self.active_popup = None;
        } else {
            self.active_popup = Some(ActivePopup::exit_confirmation());
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

    fn selected_command(&self) -> Option<&presenters::command::Presenter> {
        self.commands_state
            .selected()
            .and_then(|index| self.commands.get(index))
    }

    fn copy_to_clipboard_parameters(
        &self,
    ) -> Option<parameters::powershell::copy_to_clipboard::Parameters> {
        self.selected_command().map(|command| {
            parameters::powershell::copy_to_clipboard::Parameters {
                workspace_id: self.workspace.id.clone(),
                command_id: command.id.clone(),
            }
        })
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
    }

    fn start_windows_terminal(&mut self) {
        self.redirect = Some(Route::Powershell(
            routes::powershell::Route::StartWindowsTerminal(
                self.start_windows_terminal_parameters(),
            ),
        ));
    }

    fn powershell_set_no_exit(&mut self) {
        self.powershell_settings.set_no_exit();
    }

    fn powershell_unset_no_exit(&mut self) {
        self.powershell_settings.unset_no_exit();
    }

    pub fn new(parameters: ModelParameters) -> Result<Self> {
        let ModelParameters {
            commands,
            page_number,
            page_size,
            search_query,
            workspace,
        } = parameters;

        let mut commands_state = widgets::list::State::default();

        if !commands.is_empty() {
            commands_state.select_first();
        }

        let model = Self {
            is_running: true,
            commands,
            workspace,
            redirect: None,
            search: Input::new(search_query),
            commands_state,
            powershell_settings: PowershellSettings { no_exit: true },
            active_popup: None,
            page_number,
            page_size,
        };

        Ok(model)
    }

    fn select_next(&mut self) {
        if let Some(popup) = self.active_popup.as_mut() {
            match popup {
                ActivePopup::CommandPalette(popup) => popup.select_next(),
                ActivePopup::ExitConfirmation(_popup) => {}
            };
        } else {
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
                            search_query: self.search_query(),
                            workspace_id: self.workspace.id.clone(),
                            page_number: self.page_number + 1,
                            page_size: self.page_size,
                        },
                    ),
                )));

                return;
            }

            self.commands_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if let Some(popup) = self.active_popup.as_mut() {
            match popup {
                ActivePopup::CommandPalette(popup) => popup.select_previous(),
                ActivePopup::ExitConfirmation(_popup) => {}
            };
        } else {
            let Some(index) = self.commands_state.selected() else {
                if self.page_number != 0 {
                    self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
                        routes::workspaces::commands::Route::List(
                            parameters::workspaces::commands::list::Parameters {
                                search_query: self.search_query(),
                                workspace_id: self.workspace.id.clone(),
                                page_number: self.page_number - 1,
                                page_size: self.page_size,
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
                            search_query: self.search_query(),
                            workspace_id: self.workspace.id.clone(),
                            page_number: self.page_number - 1,
                            page_size: self.page_size,
                        },
                    ),
                )));

                return;
            }

            self.commands_state.select_previous();
        }
    }

    fn submit(&mut self) {
        if let Some(active_popup) = &mut self.active_popup {
            match active_popup {
                ActivePopup::CommandPalette(popup) => {
                    let Some(action) = popup.action() else {
                        return;
                    };

                    use components::command_palette::Action;

                    match action {
                        Action::CopyToClipboard => self.copy_to_clipboard(),
                        Action::DeleteWorkspace => {
                            self.redirect =
                                Some(Route::Workspaces(routes::workspaces::Route::Delete(
                                    parameters::workspaces::delete::Parameters {
                                        id: self.workspace.id.clone(),
                                    },
                                )))
                        }
                        Action::NewCommand => {
                            self.redirect =
                                Some(Route::Workspaces(routes::workspaces::Route::Commands(
                                    routes::workspaces::commands::Route::New(
                                        parameters::workspaces::commands::new::Parameters {
                                            workspace_id: self.workspace.id.clone(),
                                        },
                                    ),
                                )))
                        }
                        Action::ListWorkspaces => {
                            self.redirect = Some(Route::Workspaces(
                                routes::workspaces::Route::List(Default::default()),
                            ));
                        }
                        Action::EditWorkspace => {
                            self.redirect =
                                Some(Route::Workspaces(routes::workspaces::Route::Edit(
                                    parameters::workspaces::edit::Parameters {
                                        id: self.workspace.id.clone(),
                                    },
                                )))
                        }
                        Action::SetPowershellNoExit => self.powershell_set_no_exit(),
                        Action::UnsetPowerShellNoExit => self.powershell_unset_no_exit(),
                        Action::StartWindowsTerminal => self.start_windows_terminal(),
                        Action::DeleteCommand | Action::EditCommand | Action::NewWorkspace => {}
                    }
                }
                ActivePopup::ExitConfirmation(_popup) => self.exit(),
            }

            self.active_popup = None;

            return;
        }

        let Some(command) = self.command() else {
            return;
        };

        self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::Get(
                parameters::workspaces::commands::get::Parameters {
                    workspace_id: self.workspace.id.clone(),
                    command_id: command.id.clone(),
                },
            ),
        )));
    }

    fn command(&self) -> Option<&presenters::command::Presenter> {
        self.commands_state
            .selected()
            .and_then(|i| self.commands.get(i))
    }

    fn enter_char(&mut self, c: char) {
        self.search.enter_char(c);

        self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    search_query: self.search_query(),
                    workspace_id: self.workspace.id.clone(),
                    page_number: 0,
                    page_size: self.page_size,
                },
            ),
        )));
    }

    fn search_query(&self) -> String {
        self.search.value().to_string()
    }

    fn delete_char(&mut self) {
        self.search.delete_char();

        self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    search_query: self.search_query(),
                    workspace_id: self.workspace.id.clone(),
                    page_number: 0,
                    page_size: self.page_size,
                },
            ),
        )));
    }

    fn delete_all_chars(&mut self) {
        self.search.delete_all_chars();

        self.redirect = Some(Route::Workspaces(routes::workspaces::Route::Commands(
            routes::workspaces::commands::Route::List(
                parameters::workspaces::commands::list::Parameters {
                    search_query: self.search_query(),
                    workspace_id: self.workspace.id.clone(),
                    page_number: 0,
                    page_size: self.page_size,
                },
            ),
        )));
    }

    fn move_cursor_left(&mut self) {
        self.search.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.search.move_cursor_right();
    }
}
