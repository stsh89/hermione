use crate::{
    models::{
        ListWorkspacesModel, ListWorkspacesModelParameters, Model, NewWorkspaceModel, Redirect,
    },
    router::{CreateWorkspaceParameters, Router},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    model: Model,
}

impl App {
    fn create_workspace(&mut self, parameters: CreateWorkspaceParameters) -> &mut Model {
        let CreateWorkspaceParameters { name } = parameters;
        let _ = format!("{name}");

        todo!()
    }

    fn list_workspaces(&mut self) -> &mut Model {
        if matches!(self.model, Model::ListWorkspaces(_)) {
            return &mut self.model;
        }

        self.model = list_workspaces();

        &mut self.model
    }

    fn new_workspace(&mut self) -> &mut Model {
        if matches!(self.model, Model::NewWorkspace(_)) {
            return &mut self.model;
        }

        self.model = new_workspace();

        &mut self.model
    }

    fn model(&mut self, route: &Router) -> &mut Model {
        match route {
            Router::ListWorkspaces => self.list_workspaces(),
            Router::NewWorkspace => self.new_workspace(),
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters.clone()),
        }
    }

    pub fn new() -> Self {
        Self {
            model: list_workspaces(),
        }
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        let mut router = Some(Router::ListWorkspaces);

        while let Some(route) = router.as_ref() {
            let model = self.model(route);

            // Render the current view
            terminal.draw(|f| model.view(f))?;

            // Handle events and map to a Message
            let mut current_msg = model.handle_event()?;

            // Process updates as long as they return a non-None message
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

fn list_workspaces() -> Model {
    let model = ListWorkspacesModel::new(ListWorkspacesModelParameters { workspaces: vec![] });

    Model::ListWorkspaces(model)
}

fn new_workspace() -> Model {
    let model = NewWorkspaceModel::new();

    Model::NewWorkspace(model)
}
