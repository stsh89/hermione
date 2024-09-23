use ratatui::Frame;

use crate::{
    models::{Message, Redirect},
    router::Router,
    Result,
};

pub struct CreateWorkspaceModel {
    name: String,
}

pub struct CreateWorkspaceModelParameters {
    pub name: String,
}

impl CreateWorkspaceModel {
    pub fn new(parameters: CreateWorkspaceModelParameters) -> Self {
        let CreateWorkspaceModelParameters { name } = parameters;

        Self { name }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        Ok(None)
    }

    pub fn redirect(&self) -> Option<Redirect> {
        Some(Redirect::Route(Router::ListWorkspaces))
    }

    pub fn update(&mut self, _message: Message) -> Result<Option<Message>> {
        Ok(None)
    }

    pub fn view(&self, frame: &mut Frame) {}
}
