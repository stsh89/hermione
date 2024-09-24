use crate::{
    clients,
    entities::Workspace,
    models::{
        command_palette::{self, CommandPaletteModel, CommandPaletteModelParameters},
        CreateWorkspaceModel, CreateWorkspaceModelParameters, GetWorkspaceModel,
        GetWorkspaceModelParameters, ListWorkspacesModel, ListWorkspacesModelParameters, Model,
        NewCommandModel, NewWorkspaceModel,
    },
    router::{
        CommandPaletteParameters, CreateWorkspaceParameters, GetWorkspaceParameters,
        ListWorkspacesParameters, Router,
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
                let filter = search_query.to_lowercase();

                if !filter.is_empty() {
                    workspaces = workspaces
                        .into_iter()
                        .filter(|w| w.name.to_lowercase().contains(&filter))
                        .collect();
                }

                Model::ListWorkspaces(ListWorkspacesModel::new(ListWorkspacesModelParameters {
                    workspaces,
                    search_query,
                }))
            }
            Router::NewWorkspace => Model::NewWorkspace(NewWorkspaceModel::new()),
            Router::NewCommand => Model::NewCommand(NewCommandModel::new()),
            Router::CreateWorkspace(parameters) => {
                let CreateWorkspaceParameters { name } = parameters;
                self.organizer.add_workspace(name.to_string())?;

                Model::CreateWorkspace(CreateWorkspaceModel::new(CreateWorkspaceModelParameters {
                    name: name.to_string(),
                }))
            }
            Router::CommandPalette(paarameters) => {
                let CommandPaletteParameters { actions: commands } = paarameters;
                let commands = commands
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<command_palette::Action>>>()?;

                Model::CommandPalette(CommandPaletteModel::new(CommandPaletteModelParameters {
                    commands,
                    back: self.route.clone(),
                }))
            }
            Router::GetWorkspace(parameters) => {
                let GetWorkspaceParameters {
                    number,
                    commands_search_query,
                } = parameters;

                let workspace = self.organizer.get_workspace(number)?;
                let filter = commands_search_query.to_lowercase();

                let commands = if !filter.is_empty() {
                    workspace
                        .commands
                        .into_iter()
                        .filter(|c| c.name.to_lowercase().contains(&filter))
                        .collect()
                } else {
                    workspace.commands
                };

                let workspace = Workspace {
                    commands,
                    ..workspace
                };

                self.organizer.promote_workspace(workspace.number)?;

                Model::GetWorkspace(GetWorkspaceModel::new(GetWorkspaceModelParameters {
                    workspace,
                    commands_search_query,
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
        let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
            workspaces,
            search_query: String::new(),
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
                self.update_model_and_route(route.clone())?;
            }
        }

        self.organizer.save()?;

        Ok(())
    }
}
