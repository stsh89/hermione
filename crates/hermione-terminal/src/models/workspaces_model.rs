use super::Workspace;
use crate::{
    coordinator::{WorkspaceId, DEFAULT_PAGE_SIZE},
    layouts::{SearchListLayout, WideLayout},
    themes::{Theme, HIGHLIGHT_SYMBOL},
    tui::{EventHandler, Input, Model},
    widgets::FormField,
    EditWorkspaceParams, ListWorkspaceCommandsParams, ListWorkspacesParams, Message, Result, Route,
    WorkspacesRoute,
};
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph},
    Frame,
};
use std::num::NonZeroU32;

pub struct WorkspacesModel {
    is_running: bool,
    redirect: Option<Route>,
    search: Search,
    theme: Theme,
    workspaces: Workspaces,
}

pub struct WorkspacesModelParameters {
    pub page_number: Option<NonZeroU32>,
    pub page_size: Option<NonZeroU32>,
    pub search_query: Option<String>,
    pub theme: Theme,
    pub workspaces: Vec<Workspace>,
}

#[derive(Default)]
struct Search {
    input: Input,
    enabled: bool,
}

#[derive(Default)]
struct Workspaces {
    items: Vec<Workspace>,
    page_number: Option<NonZeroU32>,
    page_size: Option<NonZeroU32>,
    state: ListState,
}

struct WorkspacesParameters {
    items: Vec<Workspace>,
    page_number: Option<NonZeroU32>,
    page_size: Option<NonZeroU32>,
}

enum Action {
    Redirect(RedirectAction),
    ReloadWorkspaces,
    Search(SearchAction),
    SelectNextWorkspace,
    SelectPreviousWorkspace,
    Stop,
}

enum RedirectAction {
    EditWorkspace(WorkspaceId),
    NewWorkspace,
    Commands(WorkspaceId),
}

enum SearchAction {
    Activate,
    Clear,
    Confirm,
    Deactivate,
    DeleteChar,
    EnterChar(char),
}

enum InputMode {
    Normal,
    Input,
}

struct MessageHandler {
    workspace_id: Option<WorkspaceId>,
    input_mode: InputMode,
}

struct MessageHandlerParameters {
    input_mode: InputMode,
    workspace_id: Option<WorkspaceId>,
}

impl Search {
    fn disable(&mut self) {
        self.enabled = false;
    }

    fn enable(&mut self) {
        self.enabled = true;
    }
    fn new(search_query: Option<String>) -> Self {
        let mut search = Search::default();

        if let Some(query) = search_query {
            search.input = Input::new(query);
            search.enabled = true;
        }

        search
    }

    fn widget(&self, theme: &Theme) -> FormField<'_> {
        let mut field = FormField::default()
            .name("Search")
            .value(self.input.value())
            .set_background_color(theme.background_color)
            .set_foreground_color(theme.foreground_color);

        if self.enabled {
            field = field.set_foreground_color(theme.input_color);
        }

        field
    }
}

impl Workspaces {
    fn new(parameters: WorkspacesParameters) -> Self {
        let WorkspacesParameters {
            items,
            page_number,
            page_size,
        } = parameters;

        let mut workspaces = Workspaces {
            page_number,
            page_size,
            ..Default::default()
        };

        if !items.is_empty() {
            workspaces.items = items;
            workspaces.state.select_first();
        };

        workspaces
    }

    fn selected_id(&self) -> Option<WorkspaceId> {
        self.state.selected().map(|i| self.items[i].id)
    }
}

impl InputMode {
    fn is_normal(&self) -> bool {
        matches!(self, InputMode::Normal)
    }

    fn is_input(&self) -> bool {
        matches!(self, InputMode::Input)
    }

    fn widget(&self, theme: &Theme) -> Paragraph<'_> {
        let mut text = Text::default();

        match self {
            Self::Normal => {
                text.push_span("Press ");
                text.push_span(Span::from("q ").fg(theme.highlight_color));
                text.push_span("to quit, ");
                text.push_span(Span::from("/ ").fg(theme.highlight_color));
                text.push_span("to enter search mode, ");
                text.push_span(Span::from("n ").fg(theme.highlight_color));
                text.push_span("to create new workspace, ");
                text.push_span(Span::from("c ").fg(theme.highlight_color));
                text.push_span("to list workspace commmands");
            }
            Self::Input => {
                text.push_span("Press ");
                text.push_span(Span::from("Esc ").fg(theme.highlight_color));
                text.push_span("to discard search, ");
                text.push_span(Span::from("Enter ").fg(theme.highlight_color));
                text.push_span("to enter normal mode");
            }
        }

        Paragraph::new(text)
            .bg(theme.status_bar_background_color)
            .fg(theme.status_bar_foreground_color)
    }
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
        let mut maybe_action = MessageHandler::new(MessageHandlerParameters {
            input_mode: self.input_mode(),
            workspace_id: self.workspaces.selected_id(),
        })
        .handle_message(message);

        while let Some(action) = maybe_action {
            maybe_action = self.handle_update(action);
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        let [list_area, input_area] = SearchListLayout::new().areas(main_area);

        let block = Block::default().borders(Borders::all());

        let list = List::new(&self.workspaces.items)
            .block(block)
            .highlight_symbol(HIGHLIGHT_SYMBOL)
            .bg(self.theme.background_color)
            .fg(self.theme.foreground_color)
            .highlight_style(self.theme.highlight_color);

        frame.render_stateful_widget(list, list_area, &mut self.workspaces.state);

        frame.render_widget(self.input_mode().widget(&self.theme), status_bar_area);
        frame.render_widget(self.search.widget(&self.theme), input_area);

        if self.search.enabled {
            frame.set_cursor_position(Position::new(
                input_area.x + self.search.input.character_index() as u16 + 1,
                input_area.y + 1,
            ));
        }
    }
}

impl WorkspacesModel {
    fn handle_update(&mut self, action: Action) -> Option<Action> {
        let mut next_action = None;

        match action {
            Action::Redirect(action) => match action {
                RedirectAction::EditWorkspace(id) => {
                    self.set_redirect(EditWorkspaceParams { id }.into())
                }
                RedirectAction::NewWorkspace => self.set_redirect(WorkspacesRoute::New.into()),
                RedirectAction::Commands(workspace_id) => self.set_redirect(
                    ListWorkspaceCommandsParams {
                        workspace_id,
                        search_query: "".into(),
                        page_number: None,
                        page_size: self.workspaces.page_size,
                        powershell_no_exit: false,
                    }
                    .into(),
                ),
            },
            Action::ReloadWorkspaces => self.reload(),
            Action::Search(action) => match action {
                SearchAction::Activate => self.search.enable(),
                SearchAction::Clear => {
                    self.search.input.delete_all_chars();
                    next_action = Some(Action::ReloadWorkspaces);
                }
                SearchAction::Confirm => self.search.disable(),
                SearchAction::Deactivate => {
                    self.search.disable();
                    next_action = Some(Action::ReloadWorkspaces);
                }
                SearchAction::DeleteChar => {
                    self.search.input.delete_char();
                    next_action = Some(Action::ReloadWorkspaces);
                }
                SearchAction::EnterChar(c) => {
                    self.search.input.enter_char(c);
                    next_action = Some(Action::ReloadWorkspaces);
                }
            },
            Action::SelectNextWorkspace => next_action = self.select_next_workspace(),
            Action::SelectPreviousWorkspace => next_action = self.select_previous_workspace(),
            Action::Stop => self.stop(),
        }

        next_action
    }

    fn reload(&mut self) {
        let search_query = if self.search.enabled {
            Some(self.search.input.value().to_string())
        } else {
            None
        };

        self.set_redirect(
            ListWorkspacesParams {
                search_query,
                page_number: NonZeroU32::new(1),
                page_size: self.workspaces.page_size,
            }
            .into(),
        )
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    fn stop(&mut self) {
        self.is_running = false;
    }

    fn input_mode(&self) -> InputMode {
        if self.search.enabled {
            InputMode::Input
        } else {
            InputMode::Normal
        }
    }

    pub fn new(parameters: WorkspacesModelParameters) -> Self {
        let WorkspacesModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
            theme,
        } = parameters;

        let search = Search::new(search_query);

        let workspaces = Workspaces::new(WorkspacesParameters {
            items: workspaces,
            page_number,
            page_size,
        });

        Self {
            is_running: true,
            redirect: None,
            search,
            theme,
            workspaces,
        }
    }

    fn select_next_workspace(&mut self) -> Option<Action> {
        if self.workspaces.items.is_empty() {
            return None;
        }

        if let Some(index) = self.workspaces.state.selected() {
            if (index + 1) == self.workspaces.page_size.unwrap_or(DEFAULT_PAGE_SIZE).get() as usize
            {
                self.workspaces.page_number.and_then(|n| n.checked_add(1));
                return Some(Action::ReloadWorkspaces);
            }
        }

        self.workspaces.state.select_next();

        None
    }

    fn select_previous_workspace(&mut self) -> Option<Action> {
        if self.workspaces.items.is_empty() {
            return None;
        }

        if let Some(index) = self.workspaces.state.selected() {
            if index == 0 && self.workspaces.page_number.map(|n| n.get()).unwrap_or(1) == 1 {
                return None;
            }

            if index == 1 {
                self.workspaces
                    .page_number
                    .and_then(|n| NonZeroU32::new(n.get() - 1));
                return Some(Action::ReloadWorkspaces);
            }
        }

        None
    }
}

impl MessageHandler {
    fn cancel(self) -> Option<Action> {
        if self.input_mode.is_input() {
            Some(Action::Search(SearchAction::Deactivate))
        } else {
            None
        }
    }

    fn delete_all_chars(self) -> Option<Action> {
        if self.input_mode.is_input() {
            Some(Action::Search(SearchAction::Clear))
        } else {
            None
        }
    }

    fn delete_char(self) -> Option<Action> {
        if self.input_mode.is_input() {
            Some(Action::Search(SearchAction::DeleteChar))
        } else {
            None
        }
    }

    fn enter_char(self, c: char) -> Option<Action> {
        if self.input_mode.is_input() {
            return Some(Action::Search(SearchAction::EnterChar(c)));
        }

        match c {
            '/' => Some(Action::Search(SearchAction::Activate)),
            'j' => Some(Action::SelectNextWorkspace),
            'k' => Some(Action::SelectPreviousWorkspace),
            'q' => Some(Action::Stop),
            'n' => Some(Action::Redirect(RedirectAction::NewWorkspace)),
            'c' => self
                .workspace_id
                .map(|id| Action::Redirect(RedirectAction::Commands(id))),
            _ => None,
        }
    }

    fn handle_message(self, message: Message) -> Option<Action> {
        match message {
            Message::Cancel => self.cancel(),
            Message::DeleteAllChars => self.delete_all_chars(),
            Message::DeleteChar => self.delete_char(),
            Message::EnterChar(c) => self.enter_char(c),
            Message::ExecuteCommand => None,
            Message::MoveCusorLeft => None,
            Message::MoveCusorRight => None,
            Message::SelectNext => self.select_next(),
            Message::SelectPrevious => self.select_previous(),
            Message::Submit => self.submit(),
            Message::Tab => None,
        }
    }

    fn new(parameters: MessageHandlerParameters) -> Self {
        let MessageHandlerParameters {
            input_mode,
            workspace_id,
        } = parameters;

        Self {
            workspace_id,
            input_mode,
        }
    }

    fn select_next(self) -> Option<Action> {
        if self.input_mode.is_normal() {
            Some(Action::SelectNextWorkspace)
        } else {
            None
        }
    }

    fn select_previous(self) -> Option<Action> {
        if self.input_mode.is_normal() {
            Some(Action::SelectPreviousWorkspace)
        } else {
            None
        }
    }

    fn submit(self) -> Option<Action> {
        let mut action = None;

        match self.input_mode {
            InputMode::Normal => {
                if let Some(workspace_id) = self.workspace_id {
                    action = Some(Action::Redirect(RedirectAction::EditWorkspace(
                        workspace_id,
                    )));
                };
            }
            InputMode::Input => action = Some(Action::Search(SearchAction::Confirm)),
        }

        action
    }
}
