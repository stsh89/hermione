use crate::{
    clients,
    models::{
        CreateWorkspaceModel, CreateWorkspaceModelParameters, ListWorkspacesModel,
        ListWorkspacesModelParameters, Model, NewWorkspaceModel, NewWorkspaceModelParameters,
    },
    router::{CreateWorkspaceParameters, ListWorkspacesParameters, Router},
    Result,
};
use ratatui::{backend::Backend, Terminal};

pub struct App {
    route: Router,
    model: Model,
    organizer: clients::organizer::Client,
}

pub struct AppParameters {
    pub organizer: clients::organizer::Client,
}

impl App {
    fn update_model(&mut self) -> Result<&mut Model> {
        let route = self.route.clone();

        let model = match &self.route {
            Router::ListWorkspaces(parameters) => {
                let ListWorkspacesParameters { search_query } = parameters;
                let mut workspaces = self.organizer.list_workspaces();

                if !search_query.is_empty() {
                    workspaces = workspaces
                        .into_iter()
                        .filter(|w| w.name.to_lowercase().contains(search_query))
                        .collect();
                }

                Model::ListWorkspaces(ListWorkspacesModel::new(ListWorkspacesModelParameters {
                    workspaces,
                    route,
                }))
            }
            Router::NewWorkspace => {
                Model::NewWorkspace(NewWorkspaceModel::new(NewWorkspaceModelParameters {
                    route,
                }))
            }
            Router::CreateWorkspace(parameters) => {
                let CreateWorkspaceParameters { name } = parameters;
                self.organizer.add_workspace(name.to_string())?;

                Model::CreateWorkspace(CreateWorkspaceModel::new(CreateWorkspaceModelParameters {
                    name: name.to_string(),
                    route,
                }))
            }
        };

        self.model = model;

        Ok(&mut self.model)
    }

    pub fn new(parameters: AppParameters) -> Self {
        let AppParameters { organizer } = parameters;
        let workspaces = organizer.list_workspaces();
        let route = Router::ListWorkspaces(ListWorkspacesParameters::default());
        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            route: route.clone(),
        });

        Self {
            model: Model::ListWorkspaces(model),
            route,
            organizer,
        }
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.model.is_running() {
            terminal.draw(|f| self.model.view(f))?;

            let mut current_msg = self.model.handle_event()?;

            while current_msg.is_some() {
                current_msg = self.model.update(current_msg.unwrap())?;
            }

            if let Some(route) = self.model.redirect() {
                self.route = route.clone();
                self.update_model()?;
            }
        }

        self.organizer.save()?;

        Ok(())
    }
}
