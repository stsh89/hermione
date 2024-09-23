use crate::{
    clients,
    models::{CreateWorkspaceModel, CreateWorkspaceModelParameters, Model, Redirect},
    router::{CreateWorkspaceParameters, Router},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    model: Model,
    organizer: clients::organizer::Client,
}

pub struct AppParameters {
    pub organizer: clients::organizer::Client,
}

impl App {
    fn create_workspace(&mut self, parameters: &CreateWorkspaceParameters) -> Result<&mut Model> {
        let CreateWorkspaceParameters { name } = parameters;
        self.organizer.add_workspace(name.to_string())?;

        self.model =
            Model::CreateWorkspace(CreateWorkspaceModel::new(CreateWorkspaceModelParameters {
                name: name.to_string(),
            }));

        Ok(&mut self.model)
    }

    fn list_workspaces(&mut self) -> &mut Model {
        if self.model.is_list_workspaces() {
            return &mut self.model;
        }

        let workspaces = self.organizer.list_workspaces();
        self.model = Model::list_workspaces(workspaces);

        &mut self.model
    }

    fn model(&mut self, route: &Router) -> Result<&mut Model> {
        match route {
            Router::ListWorkspaces => Ok(self.list_workspaces()),
            Router::NewWorkspace => Ok(self.new_workspace()),
            Router::CreateWorkspace(parameters) => self.create_workspace(parameters),
        }
    }

    pub fn new(parameters: AppParameters) -> Self {
        let AppParameters { organizer } = parameters;
        let workspaces = organizer.list_workspaces();

        Self {
            model: Model::list_workspaces(workspaces),
            organizer,
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
            let model = self.model(route)?;

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

        self.organizer.save()?;

        Ok(())
    }
}
