mod dispatcher;
mod frame;
mod state;

use crate::{
    coordinator::Workspace,
    themes::Theme,
    tui::{EventHandler, Model},
    CreateWorkspaceParams, ListWorkspacesParams, Message, Result, Route, UpdateWorkspaceParams,
};
use frame::View;
use ratatui::Frame;
use state::ModelState;

#[derive(Default)]
pub struct WorkspaceFormModel {
    state: ModelState,
}

pub struct WorkspaceModelParameters {
    pub workspace: Option<Workspace>,
    pub theme: Theme,
}

impl WorkspaceFormModel {
    pub fn new(parameters: WorkspaceModelParameters) -> Self {
        let WorkspaceModelParameters { workspace, theme } = parameters;

        let mut state = ModelState::default();

        if let Some(workspace) = workspace {
            state.set_workspace_id(workspace.id);
            state.set_workspace_name(workspace.name);
            state.set_workspace_location(workspace.location);
        }

        state.set_theme(theme);

        Self { state }
    }
}

impl Model for WorkspaceFormModel {
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
        let view = View {
            name: self.state.workspace_name(),
            location: self.state.workspace_location(),
            theme: self.state.theme(),
            name_is_active: self.state.is_name_input_active(),
            location_is_active: self.state.is_location_input_active(),
            is_normal_mode: self.state.is_in_normal_mode(),
            is_input_mode: self.state.is_in_input_mode(),
            name_character_index: self.state.workspace_name_character_index(),
            location_character_index: self.state.workspace_location_character_index(),
        };

        frame::render(frame, view);
    }
}
