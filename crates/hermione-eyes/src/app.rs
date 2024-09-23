use crate::{
    models::{Model, Redirect},
    router::{CreateWorkspaceParameters, Router},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    model: Model,
}

impl App {
    fn create_workspace(&mut self, parameters: &CreateWorkspaceParameters) -> &mut Model {
        let CreateWorkspaceParameters { name } = parameters;
        let _ = format!("{name}");

        todo!()
    }

    fn list_workspaces(&mut self) -> &mut Model {
        if self.model.is_list_workspaces() {
            return &mut self.model;
        }

        self.model = Model::list_workspaces(vec![]);

        &mut self.model
    }

    fn model(&mut self, route: &Router) -> &mut Model {
        match route {
            Router::ListWorkspaces => self.list_workspaces(),
            Router::NewWorkspace => self.new_workspace(),
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters),
        }
    }

    pub fn new() -> Self {
        Self {
            model: Model::list_workspaces(vec![]),
        }
    }

    fn new_workspace(&mut self) -> &mut Model {
        if self.model.is_new_workspace() {
            return &mut self.model;
        }

        self.model = Model::new_workspace();

        &mut self.model
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        let mut router = Some(Router::ListWorkspaces);

        while let Some(route) = router.as_ref() {
            let model = self.model(route);

            terminal.draw(|f| model.view(f))?;

            let mut current_msg = model.handle_event()?;

            while current_msg.is_some() {
                current_msg = model.update(current_msg.unwrap())?;
            }

            if let Some(redirect) = model.redirect() {
                match redirect {
                    Redirect::Exit => router = None,
                    Redirect::Route(route) => router = Some(route),
                }
            }
        }

        Ok(())
    }
}
