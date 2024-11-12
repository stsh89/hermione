use super::Workspace;
use crate::{
    coordinator::{DEFAULT_PAGE_SIZE, FIRST_PAGE},
    layouts::{SearchListLayout, WideLayout},
    smart_input::{NewSmartInputParameters, SmartInput},
    themes::{Theme, HIGHLIGHT_SYMBOL},
    tui::{EventHandler, Model},
    BackupCredentialsRoute, DeleteWorkspaceParams, EditWorkspaceParams, Error,
    ListWorkspaceCommandsParams, ListWorkspacesParams, Message, Result, Route, WorkspacesRoute,
};
use ratatui::{
    layout::Rect,
    style::Stylize,
    text::ToText,
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};
use std::num::NonZeroU32;

pub struct WorkspacesModel {
    is_running: bool,
    redirect: Option<Route>,
    workspaces_state: ListState,
    workspaces: Vec<Workspace>,
    page_number: NonZeroU32,
    page_size: NonZeroU32,
    smart_input: SmartInput,
    search_query: String,
    theme: Theme,
}

pub struct WorkspacesModelParameters {
    pub workspaces: Vec<Workspace>,
    pub search_query: String,
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub theme: Theme,
}

impl Model for WorkspacesModel {
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
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let [list_area, input_area] = SearchListLayout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());

        let list = List::new(&self.workspaces)
            .block(block)
            .highlight_symbol(HIGHLIGHT_SYMBOL)
            .bg(self.theme.background_color)
            .fg(self.theme.foreground_color)
            .highlight_style(self.theme.highlight_color);

        frame.render_stateful_widget(list, list_area, &mut self.workspaces_state);
        self.smart_input.render(frame, input_area);
        self.render_status_bar(frame, status_bar_area);
    }
}

impl WorkspacesModel {
    fn toggle_focus(&mut self) {
        self.smart_input.autocomplete();
    }

    fn cancel(&mut self) {
        self.smart_input.reset_input();

        if !self.search_query.is_empty() {
            self.set_redirect(ListWorkspacesParams::default().into());
        }
    }

    fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn new(parameters: WorkspacesModelParameters) -> Result<Self> {
        let WorkspacesModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
            theme,
        } = parameters;

        let smart_input = SmartInput::new(NewSmartInputParameters {
            theme,
            commands: Action::all().into_iter().map(Into::into).collect(),
        });

        let mut model = Self {
            workspaces,
            redirect: None,
            workspaces_state: ListState::default(),
            is_running: true,
            page_number: page_number.unwrap_or(FIRST_PAGE),
            page_size: page_size.unwrap_or(DEFAULT_PAGE_SIZE),
            smart_input,
            search_query,
            theme,
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

    fn render_status_bar(&self, frame: &mut Frame, area: Rect) {
        let mut value = serde_json::json!({
            "screen": "Workspaces",
            "page": self.page_number,
        });

        if let Some(workspace) = self.workspace() {
            value["workspace_name"] = workspace.name.clone().into();
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

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn set_list_workspaces_redirect(&mut self, search_query: String) {
        self.redirect = Some(
            ListWorkspacesParams {
                search_query,
                page_number: None,
                page_size: Some(self.page_size),
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
                    self.set_redirect(DeleteWorkspaceParams { id: workspace.id }.into())
                }
            }
            Action::EditWorkspace => {
                if let Some(workspace) = self.workspace() {
                    self.set_redirect(EditWorkspaceParams { id: workspace.id }.into());
                }
            }
            Action::Exit => self.exit(),
            Action::ListBackupCredentials => {
                self.set_redirect(Route::BackupCredentials(BackupCredentialsRoute::List))
            }
            Action::ListCommands => {
                if let Some(workspace) = self.workspace() {
                    self.set_redirect(
                        ListWorkspaceCommandsParams {
                            workspace_id: workspace.id,
                            search_query: "".into(),
                            page_number: None,
                            page_size: Some(self.page_size),
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
        self.workspaces_state.select_next();
    }

    fn select_previous(&mut self) {
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

    fn workspace(&self) -> Option<&Workspace> {
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
    ListBackupCredentials,
}

impl Action {
    fn all() -> Vec<Self> {
        vec![
            Self::DeleteWorkspace,
            Self::EditWorkspace,
            Self::Exit,
            Self::ListCommands,
            Self::NewWorkspace,
            Self::ListBackupCredentials,
        ]
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        let action = match action {
            Action::DeleteWorkspace => "Delete workspace",
            Action::EditWorkspace => "Edit workspace",
            Action::Exit => "Exit",
            Action::ListBackupCredentials => "List backup credentials",
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
            "List backup credentials" => Ok(Self::ListBackupCredentials),
            _ => Err(anyhow::anyhow!("Unknown action: {}", value)),
        }
    }
}
