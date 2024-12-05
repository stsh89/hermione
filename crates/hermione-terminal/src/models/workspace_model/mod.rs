mod hub;
mod screen;
mod sygnal;
mod views;

use crate::{
    coordinator::Workspace,
    themes::Theme,
    tui::{EventHandler, Model},
    CreateWorkspaceParams, ListWorkspacesParams, Message, Result, Route, UpdateWorkspaceParams,
};
use ratatui::Frame;
use screen::Screen;

#[derive(Default)]
pub struct WorkspaceModel(Screen);

pub struct WorkspaceModelParameters {
    pub workspace: Option<Workspace>,
    pub theme: Theme,
}

impl WorkspaceModel {
    pub fn new(parameters: WorkspaceModelParameters) -> Self {
        let WorkspaceModelParameters { workspace, theme } = parameters;

        let mut screen = Screen::default();

        if let Some(workspace) = workspace {
            screen.set_workspace_id(workspace.id);
            screen.set_workspace_name(workspace.name);
            screen.set_workspace_location(workspace.location);
        }

        screen.set_theme(theme);

        Self(screen)
    }
}

impl Model for WorkspaceModel {
    type Message = Message;
    type Route = Route;

    fn handle_event(&self) -> Result<Option<Self::Message>> {
        EventHandler::new(|key_event| key_event.try_into().ok()).handle_event()
    }

    fn is_running(&self) -> bool {
        self.0.is_running()
    }

    fn redirect(&mut self) -> Option<Route> {
        self.0.redirect()
    }

    fn update(&mut self, message: Message) -> Result<Option<Message>> {
        if let Some(sygnal) = hub::create_sygnal(message, &self.0) {
            hub::update_screen(sygnal, &mut self.0);
        }

        Ok(None)
    }

    fn view(&mut self, frame: &mut Frame) {
        let view = views::View {
            name: self.0.workspace_name(),
            location: self.0.workspace_location(),
            theme: self.0.theme(),
            name_is_active: self.0.is_name_input_active(),
            location_is_active: self.0.is_location_input_active(),
            is_normal_mode: self.0.is_in_normal_mode(),
            is_input_mode: self.0.is_in_input_mode(),
            name_character_index: self.0.workspace_name_character_index(),
            location_character_index: self.0.workspace_location_character_index(),
        };

        views::render(frame, view);
    }
}
