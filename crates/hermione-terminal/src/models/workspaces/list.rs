use crate::{
    layouts::{self, StatusBar},
    smart_input::{NewSmartInputParameters, SmartInput},
    widgets, DeleteWorkspaceParameters, EditWorkspaceParameters, Error,
    ListWorkspaceCommandsParameters, ListWorkspacesParameters, Message, Result, Route,
    WorkspacePresenter, WorkspacesRoute, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_tui::{EventHandler, Model};
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ListWorkspacesModel {
    is_running: bool,
    redirect: Option<Route>,
    workspaces_state: widgets::list::State,
    workspaces: Vec<WorkspacePresenter>,
    page_number: u32,
    page_size: u32,
    smart_input: SmartInput,
    search_query: String,
}

pub struct ListWorkspaceModelParameters {
    pub workspaces: Vec<WorkspacePresenter>,
    pub search_query: String,
    pub page_number: u32,
    pub page_size: u32,
}

impl Model for ListWorkspacesModel {
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
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::Cancel => self.cancel(),
            Message::MoveCusorLeft => self.move_cursor_left(),
            Message::MoveCusorRight => self.move_cursor_right(),
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit()?,
            Message::Tab => self.toggle_focus(),
            Message::ExecuteCommand => {}
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = layouts::wide::Layout::new().areas(frame.area());
        let [list_area, input_area] = layouts::search_list::Layout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());
        let list = widgets::list::Widget::new(&self.workspaces).block(block);

        frame.render_stateful_widget(list, list_area, &mut self.workspaces_state);
        self.smart_input.render(frame, input_area);

        let paragraph = Paragraph::new(self.status_bar());
        frame.render_widget(paragraph, status_bar_area);
    }
}

impl ListWorkspacesModel {
    fn toggle_focus(&mut self) {
        self.smart_input.autocomplete();
    }

    fn cancel(&mut self) {
        self.smart_input.reset_input();

        if !self.search_query.is_empty() {
            self.set_redirect(ListWorkspacesParameters::default().into());
        }
    }

    fn status_bar(&self) -> String {
        let mut status_bar = StatusBar::default()
            .use_case("List workspaces")
            .page(self.page_number);

        if let Some(workspace) = self.workspace() {
            status_bar = status_bar.selector(&workspace.name);
        }

        if !self.search_query.is_empty() {
            status_bar = status_bar.search(&self.search_query);
        }

        status_bar.try_into().unwrap_or_default()
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn new(parameters: ListWorkspaceModelParameters) -> Result<Self> {
        let ListWorkspaceModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
        } = parameters;

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: widgets::list::State::default(),
            is_running: true,
            page_number,
            page_size,
            smart_input: smart_input(),
            search_query,
        };

        if !model.workspaces.is_empty() {
            model.workspaces_state.select_first();
        }

        if !model.search_query.is_empty() {
            for c in model.search_query.chars() {
                model.smart_input.enter_char(c);
            }
        }

        Ok(model)
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn set_list_workspaces_redirect(&mut self, search_query: String) {
        self.redirect = Some(
            ListWorkspacesParameters {
                search_query,
                page_number: 0,
                page_size: self.page_size,
            }
            .into(),
        );
    }

    fn submit(&mut self) -> Result<()> {
        let Some(command) = self.smart_input.command() else {
            self.smart_input.reset_input();
            return Ok(());
        };

        let action = Action::try_from(command)?;

        match action {
            Action::DeleteWorkspace => {
                if let Some(workspace) = self.workspace() {
                    self.set_redirect(
                        DeleteWorkspaceParameters {
                            id: workspace.id.clone(),
                        }
                        .into(),
                    )
                }
            }
            Action::EditWorkspace => {
                if let Some(workspace) = self.workspace() {
                    self.set_redirect(
                        EditWorkspaceParameters {
                            id: workspace.id.clone(),
                        }
                        .into(),
                    );
                }
            }
            Action::Exit => self.exit(),
            Action::ListCommands => {
                if let Some(workspace) = self.workspace() {
                    self.set_redirect(
                        ListWorkspaceCommandsParameters {
                            workspace_id: workspace.id.clone(),
                            search_query: "".into(),
                            page_number: 0,
                            page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                            powershell_no_exit: false,
                        }
                        .into(),
                    );
                }
            }
            Action::NewWorkspace => self.set_redirect(Route::Workspaces(WorkspacesRoute::New)),
        }

        Ok(())
    }

    fn select_next(&mut self) {
        let Some(index) = self.workspaces_state.selected() else {
            return;
        };

        if index == self.workspaces.len() - 1 && self.workspaces.len() == self.page_size as usize {
            self.set_redirect(
                ListWorkspacesParameters {
                    search_query: self.search_query.clone(),
                    page_number: self.page_number + 1,
                    page_size: self.page_size,
                }
                .into(),
            );

            return;
        }

        self.workspaces_state.select_next();
    }

    fn select_previous(&mut self) {
        let Some(index) = self.workspaces_state.selected() else {
            if self.page_number != 0 {
                self.set_redirect(
                    ListWorkspacesParameters {
                        search_query: self.search_query.clone(),
                        page_number: self.page_number - 1,
                        page_size: self.page_size,
                    }
                    .into(),
                );
            }

            return;
        };

        if index == 0 && self.page_number != 0 {
            self.set_redirect(
                ListWorkspacesParameters {
                    search_query: self.search_query.clone(),
                    page_number: self.page_number - 1,
                    page_size: self.page_size,
                }
                .into(),
            );

            return;
        }

        self.workspaces_state.select_previous();
    }

    fn enter_char(&mut self, c: char) {
        self.smart_input.enter_char(c);

        if let Some(search_query) = self.smart_input.search() {
            self.set_list_workspaces_redirect(search_query.into());
        };
    }

    fn delete_char(&mut self) {
        self.smart_input.delete_char();

        let Some(search_query) = self.smart_input.search() else {
            return;
        };

        self.set_list_workspaces_redirect(search_query.into());
    }

    fn delete_all_chars(&mut self) {
        self.smart_input.reset_input();

        let Some(search_query) = self.smart_input.search() else {
            return;
        };

        self.set_list_workspaces_redirect(search_query.into());
    }

    fn move_cursor_left(&mut self) {
        self.smart_input.move_cursor_left();
    }

    fn move_cursor_right(&mut self) {
        self.smart_input.move_cursor_right();
    }

    fn workspace(&self) -> Option<&WorkspacePresenter> {
        self.workspaces_state
            .selected()
            .and_then(|i| self.workspaces.get(i))
    }
}

enum Action {
    DeleteWorkspace,
    EditWorkspace,
    Exit,
    ListCommands,
    NewWorkspace,
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        let action = match action {
            Action::DeleteWorkspace => "Delete workspace",
            Action::EditWorkspace => "Edit workspace",
            Action::Exit => "Exit",
            Action::ListCommands => "List commands",
            Action::NewWorkspace => "New workspace",
        };

        action.into()
    }
}

impl TryFrom<&str> for Action {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "Delete workspace" => Ok(Self::DeleteWorkspace),
            "Edit workspace" => Ok(Self::EditWorkspace),
            "Exit" => Ok(Self::Exit),
            "List commands" => Ok(Self::ListCommands),
            "New workspace" => Ok(Self::NewWorkspace),
            _ => Err(anyhow::anyhow!("Unknown action: {}", value)),
        }
    }
}

fn smart_input() -> SmartInput {
    SmartInput::new(NewSmartInputParameters {
        commands: vec![
            Action::DeleteWorkspace.into(),
            Action::EditWorkspace.into(),
            Action::Exit.into(),
            Action::ListCommands.into(),
            Action::NewWorkspace.into(),
        ],
    })
}
