use ratatui::Frame;

use crate::{
    models::Message,
    router::{GetWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};

pub struct CreateCommandModel {
    name: String,
    program: String,
    is_running: bool,
    redirect: Option<Router>,
}

pub struct CreateCommandModelParameters {
    pub name: String,
    pub program: String,
}

impl CreateCommandModel {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn new(parameters: CreateCommandModelParameters) -> Self {
        let CreateCommandModelParameters { name, program } = parameters;

        Self {
            name,
            program,
            is_running: true,
            redirect: Some(Router::GetWorkspace(GetWorkspaceParameters {
                number: 0,
                commands_search_query: String::new(),
            })),
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
