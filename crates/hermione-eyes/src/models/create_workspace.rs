use ratatui::Frame;

use crate::{
    models::{Message, Redirect},
    router::{ListWorkspacesParameters, Router},
    Result,
};

pub struct CreateWorkspaceModel {
    name: String,
    is_running: bool,
    redirect: Option<Router>,
}

pub struct CreateWorkspaceModelParameters {
    pub name: String,
}

impl CreateWorkspaceModel {
    pub fn is_running(&self) -> bool {
        true
    }

    pub fn new(parameters: CreateWorkspaceModelParameters) -> Self {
        let CreateWorkspaceModelParameters { name } = parameters;

        Self {
            name,
            is_running: true,
            redirect: Some(Router::ListWorkspaces(ListWorkspacesParameters::default())),
        }
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        Ok(None)
    }

    pub fn redirect(&self) -> Option<&Router> {
        self.redirect.as_ref()
    }

    pub fn update(&mut self, _message: Message) -> Result<Option<Message>> {
        Ok(None)
    }

    pub fn view(&self, _frame: &mut Frame) {}
}
