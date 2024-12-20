use super::{CreateWorkspaceParams, ListWorkspacesParams, Route, UpdateWorkspaceParams};
use crate::{themes::Theme, tui::Input};
use hermione_nexus::definitions::WorkspaceId;

#[derive(Default)]
pub struct ModelState {
    active_input: InputName,
    location: Input,
    name: Input,
    redirect: Option<Route>,
    theme: Theme,
    workspace_id: Option<WorkspaceId>,
}

#[derive(Default)]
enum InputName {
    #[default]
    Name,
    Location,
}

impl ModelState {
    fn active_input_mut(&mut self) -> &mut Input {
        match self.active_input {
            InputName::Name => &mut self.name,
            InputName::Location => &mut self.location,
        }
    }

    pub fn clear_input(&mut self) {
        self.active_input_mut().delete_all_chars();
    }

    pub fn set_redirect_to_create_workspace(&mut self) {
        self.set_redirect(
            CreateWorkspaceParams {
                name: self.name.value().to_string(),
                location: self.location.value().to_string(),
            }
            .into(),
        );
    }

    pub fn delete_char(&mut self) {
        self.active_input_mut().delete_char();
    }

    pub fn enter_char(&mut self, c: char) {
        self.active_input_mut().enter_char(c);
    }

    pub fn is_name_input_active(&self) -> bool {
        matches!(self.active_input, InputName::Name)
    }

    pub fn is_location_input_active(&self) -> bool {
        matches!(self.active_input, InputName::Location)
    }

    pub fn set_redirect_to_list_workspaces(&mut self) {
        self.set_redirect(ListWorkspacesParams::default().into());
    }

    pub fn move_cursor_left(&mut self) {
        self.active_input_mut().move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.active_input_mut().move_cursor_right();
    }

    pub fn redirect(&mut self) -> Option<Route> {
        self.redirect.take()
    }

    pub fn select_next_input(&mut self) {
        self.active_input = match self.active_input {
            InputName::Name => InputName::Location,
            InputName::Location => InputName::Name,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    fn set_redirect(&mut self, route: Route) {
        self.redirect = Some(route);
    }

    pub fn set_workspace_id(&mut self, workspace_id: WorkspaceId) {
        self.workspace_id = Some(workspace_id);
    }

    pub fn set_workspace_location(&mut self, location: String) {
        self.location = Input::new(location)
    }

    pub fn set_workspace_name(&mut self, name: String) {
        self.name = Input::new(name)
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn set_redirect_to_update_workspace(&mut self, workspace_id: WorkspaceId) {
        self.set_redirect(
            UpdateWorkspaceParams {
                name: self.name.value().to_string(),
                location: self.location.value().to_string(),
                id: workspace_id,
            }
            .into(),
        );
    }

    pub fn workspace_id(&self) -> Option<WorkspaceId> {
        self.workspace_id
    }

    pub fn workspace_name(&self) -> &str {
        self.name.value()
    }

    pub fn workspace_name_character_index(&self) -> u16 {
        self.name.character_index() as u16
    }

    pub fn workspace_location(&self) -> &str {
        self.location.value()
    }

    pub fn workspace_location_character_index(&self) -> u16 {
        self.location.character_index() as u16
    }
}
