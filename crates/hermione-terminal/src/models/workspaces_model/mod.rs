mod dispatcher;
mod frame;
mod state;

use super::Workspace;
use crate::{
    themes::Theme,
    tui::{EventHandler, Model},
    EditWorkspaceParams, ListWorkspaceCommandsParams, ListWorkspacesParams, Message, Result, Route,
    WorkspacesRoute,
};
use frame::View;
use ratatui::Frame;
use state::ModelState;
use std::num::NonZeroU32;

pub struct WorkspacesModel {
    state: ModelState,
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
        self.state.is_running()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.state.redirect()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        if let Some(action) = dispatcher::maybe_create_action(message, &self.state) {
            dispatcher::dispatch(action, &mut self.state);
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        // let [main_area, status_bar_area] = WideLayout::new().areas(frame.area());
        // let [list_area, input_area] = SearchListLayout::new().areas(main_area);

        // let block = Block::default().borders(Borders::all());

        // let list = List::new(&self.workspaces)
        //     .block(block)
        //     .highlight_symbol(HIGHLIGHT_SYMBOL)
        //     .bg(self.theme.background_color)
        //     .fg(self.theme.foreground_color)
        //     .highlight_style(self.theme.highlight_color);

        // frame.render_stateful_widget(list, list_area, &mut self.workspaces_state);
        // self.smart_input.render(frame, input_area);
        // self.render_status_bar(frame, status_bar_area);

        let view = View {
            list_state: self.state.workspaces_state().clone(),
            theme: self.state.theme(),
            is_normal_mode: self.state.is_in_normal_mode(),
            is_searching: self.state.is_in_search_mode(),
            workspaces: self.state.workspaces(),
            search: self.state.search(),
            search_character_index: self.state.search_character_index(),
        };

        frame::render(frame, view);
    }
}

impl WorkspacesModel {
    pub fn new(parameters: WorkspacesModelParameters) -> Self {
        let WorkspacesModelParameters {
            workspaces,
            search_query,
            page_number,
            page_size,
            theme,
        } = parameters;

        let mut state = ModelState::default();

        if !search_query.is_empty() {
            state.activate_search_mode();
        }

        state.set_search_query(search_query);
        state.set_theme(theme);
        state.set_workspaces(workspaces);

        if let Some(page_number) = page_number {
            state.set_page_number(page_number);
        }

        if let Some(page_size) = page_size {
            state.set_page_size(page_size);
        }

        Self { state }
    }
}
