use crate::{
    models::{ListWorkspacesModel, ListWorkspacesModelParameters, Model},
    router::Router,
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    model: Model,
    route: Option<Router>,
}

impl App {
    fn list_workspaces(&mut self) -> &mut Model {
        if matches!(self.model, Model::ListWorkspaces(_)) {
            return &mut self.model;
        }

        self.model = list_workspaces();

        &mut self.model
    }

    fn model(&mut self, route: Router) -> &mut Model {
        match route {
            Router::ListWorkspaces => self.list_workspaces(),
        }
    }

    pub fn new() -> Self {
        Self {
            route: Some(Router::ListWorkspaces),
            model: list_workspaces(),
        }
    }

    pub fn run(mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        while let Some(route) = self.route {
            let model = self.model(route);

            // Render the current view
            terminal.draw(|f| model.view(f))?;

            // Handle events and map to a Message
            let mut current_msg = model.handle_event()?;

            // Process updates as long as they return a non-None message
            while current_msg.is_some() {
                current_msg = model.update(current_msg.unwrap())?;
            }

            self.route = model.route();
        }

        Ok(())
    }
}

fn list_workspaces() -> Model {
    let model = ListWorkspacesModel::new(ListWorkspacesModelParameters { workspaces: vec![] });

    Model::ListWorkspaces(model)
}
