use crate::{
    clients,
    models::{
        CommandPaletteModel, CommandPaletteModelParameters, CreateWorkspaceModel,
        CreateWorkspaceModelParameters, ListWorkspacesModel, ListWorkspacesModelParameters, Model,
        NewWorkspaceModel,
    },
    router::{
        CommandPaletteParameters, CreateWorkspaceParameters, ListWorkspacesParameters, Router,
    },
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
    fn update_model_and_route(&mut self, route: Router) -> Result<&mut Model> {
        let model = match route.clone() {
            Router::ListWorkspaces(parameters) => {
                let ListWorkspacesParameters { search_query } = parameters;
                let mut workspaces = self.organizer.list_workspaces();

                if !search_query.is_empty() {
                    workspaces = workspaces
                        .into_iter()
                        .filter(|w| w.name.to_lowercase().contains(&search_query))
                        .collect();
                }

                Model::ListWorkspaces(ListWorkspacesModel::new(ListWorkspacesModelParameters {
                    workspaces,
                }))
            }
            Router::NewWorkspace => Model::NewWorkspace(NewWorkspaceModel::new()),
            Router::CreateWorkspace(parameters) => {
                let CreateWorkspaceParameters { name } = parameters;
                self.organizer.add_workspace(name.to_string())?;

                Model::CreateWorkspace(CreateWorkspaceModel::new(CreateWorkspaceModelParameters {
                    name: name.to_string(),
                }))
            }
            Router::CommandPalette(paarameters) => {
                let CommandPaletteParameters { commands } = paarameters;

                Model::CommandPalette(CommandPaletteModel::new(CommandPaletteModelParameters {
                    commands: commands.clone(),
                    back: self.route.clone(),
                }))
            }
        };

        self.model = model;
        self.route = route;

        Ok(&mut self.model)
    }

    pub fn new(parameters: AppParameters) -> Self {
        let AppParameters { organizer } = parameters;
        let workspaces = organizer.list_workspaces();
        let route = Router::ListWorkspaces(ListWorkspacesParameters::default());
        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters { workspaces });

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
                self.update_model_and_route(route.clone())?;
            }
        }

        self.organizer.save()?;

        Ok(())
    }
}
