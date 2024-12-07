use crate::{
    coordinator::{Workspace, WorkspaceId},
    themes::Theme,
    tui::Input,
};
use ratatui::widgets::ListState;
use std::num::NonZeroU32;

use super::{
    EditWorkspaceParams, ListWorkspaceCommandsParams, ListWorkspacesParams, Route, WorkspacesRoute,
};

#[derive(Default)]
pub struct ModelState {
    list_state: ListState,
    mode: Mode,
    page_number: Option<NonZeroU32>,
    page_size: Option<NonZeroU32>,
    redirect: Option<Route>,
    search_query: Input,
    theme: Theme,
    usage: UsageIndicator,
    workspaces: Vec<Workspace>,
}

#[derive(Default)]
enum Mode {
    #[default]
    Normal,
    Searching,
}

#[derive(Default)]
enum UsageIndicator {
    #[default]
    IsRunning,
    Stopped,
}

impl ModelState {
    pub fn activate_search_mode(&mut self) {
        self.mode = Mode::Searching;
    }

    pub fn delete_search_query(&mut self) {
        self.search_query.delete_all_chars();

        self.set_redirect(
            ListWorkspacesParams {
                search_query: self.search_query.value().to_string(),
                page_number: NonZeroU32::new(1),
                page_size: self.page_size,
            }
            .into(),
        );
    }

    pub fn edit_search_query(&mut self) {
        self.search_query.delete_char();

        self.set_redirect(
            ListWorkspacesParams {
                search_query: self.search_query.value().to_string(),
                page_number: NonZeroU32::new(1),
                page_size: self.page_size,
            }
            .into(),
        );
    }

    pub fn enter_search_mode(&mut self) {
        self.mode = Mode::Searching;
    }

    pub fn exit_search_mode(&mut self) {
        self.mode = Mode::Normal;

        self.set_redirect(
            ListWorkspacesParams {
                search_query: String::new(),
                page_number: NonZeroU32::new(1),
                page_size: self.page_size,
            }
            .into(),
        );
    }

    pub fn is_in_normal_mode(&self) -> bool {
        matches!(self.mode, Mode::Normal)
    }

    pub fn is_in_search_mode(&self) -> bool {
        matches!(self.mode, Mode::Searching)
    }

    pub fn is_first_workspace_selected(&self) -> bool {
        let Some(index) = self.list_state.selected() else {
            return false;
        };

        index == 0
    }

    pub fn is_last_workspace_selected(&self) -> bool {
        let Some(index) = self.list_state.selected() else {
            return false;
        };

        self.workspaces.len() == (index + 1)
    }

    pub fn is_running(&self) -> bool {
        matches!(self.usage, UsageIndicator::IsRunning)
    }

    pub fn move_search_query_cursor_left(&mut self) {
        self.search_query.move_cursor_left();
    }

    pub fn move_search_query_cursor_right(&mut self) {
        self.search_query.move_cursor_right();
    }

    fn page_number(&self) -> NonZeroU32 {
        self.page_number
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) })
    }

    fn page_size(&self) -> NonZeroU32 {
        self.page_size
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(100) })
    }

    pub fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    pub fn search(&self) -> &str {
        self.search_query.value()
    }

    pub fn search_character_index(&self) -> u16 {
        self.search_query.character_index() as u16
    }

    pub fn select_next_workspace(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_previous_workspace(&mut self) {
        self.list_state.select_previous();
    }

    pub fn selected_workspace_id(&self) -> Option<WorkspaceId> {
        self.list_state
            .selected()
            .and_then(|i| self.workspaces.get(i))
            .map(|workspace| workspace.id)
    }

    pub fn set_commands_redirect(&mut self, workspace_id: WorkspaceId) {
        self.set_redirect(
            ListWorkspaceCommandsParams {
                workspace_id,
                search_query: "".into(),
                page_number: None,
                page_size: Some(self.page_size()),
                powershell_no_exit: false,
            }
            .into(),
        );
    }

    pub fn set_next_page_redirect(&mut self) {
        if self.page_size().get() > self.workspaces.len() as u32 {
            return;
        }

        self.set_redirect(
            ListWorkspacesParams {
                search_query: String::new(),
                page_number: self.page_number().checked_add(1),
                page_size: Some(self.page_size()),
            }
            .into(),
        );
    }

    pub fn set_new_workspace_redirect(&mut self) {
        self.set_redirect(Route::Workspaces(WorkspacesRoute::New));
    }

    pub fn set_page_number(&mut self, page_number: NonZeroU32) {
        self.page_number = Some(page_number);
    }

    pub fn set_page_size(&mut self, page_size: NonZeroU32) {
        self.page_size = Some(page_size);
    }

    pub fn set_previous_page_redirect(&mut self) {
        if self.page_number().get() == 1 {
            return;
        }

        self.set_redirect(
            ListWorkspacesParams {
                search_query: String::new(),
                page_number: Some(unsafe {
                    NonZeroU32::new_unchecked(self.page_number().get() - 1)
                }),
                page_size: Some(self.page_size()),
            }
            .into(),
        );
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = Input::new(query);
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn set_workspace_redirect(&mut self, id: WorkspaceId) {
        self.set_redirect(EditWorkspaceParams { id }.into());
    }

    pub fn set_workspaces(&mut self, workspaces: Vec<Workspace>) {
        if !workspaces.is_empty() {
            self.list_state.select_first();
        }

        self.workspaces = workspaces;
    }

    pub fn stop(&mut self) {
        self.usage = UsageIndicator::Stopped;
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn update_search_query(&mut self, c: char) {
        self.search_query.enter_char(c);

        self.set_redirect(
            ListWorkspacesParams {
                search_query: self.search_query.value().to_string(),
                page_number: NonZeroU32::new(1),
                page_size: self.page_size,
            }
            .into(),
        );
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }

    pub fn workspaces_state(&mut self) -> &ListState {
        &self.list_state
    }
}
