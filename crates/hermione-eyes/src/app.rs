use crate::{
    clients::{self, organizer::CommandParameters},
    entities::Workspace,
    models::{
        command_palette::{self, CommandPaletteModel, CommandPaletteModelParameters},
        CreateCommandModel, CreateCommandModelParameters, CreateWorkspaceModel,
        CreateWorkspaceModelParameters, GetCommandModel, GetCommandModelParameters,
        GetWorkspaceModel, GetWorkspaceModelParameters, ListWorkspacesModel,
        ListWorkspacesModelParameters, Model, NewCommandModel, NewWorkspaceModel,
    },
    router::{
        CommandPaletteParameters, CreateCommandParameters, CreateWorkspaceParameters,
        GetCommandParameters, GetWorkspaceParameters, ListWorkspacesParameters, Router,
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
            Router::CreateCommand(parameters) => {
                let CreateCommandParameters { name, program } = parameters;
                self.organizer.add_command(CommandParameters {
                    workspace_number: 0,
                    name: name.clone(),
                    program: program.clone(),
                })?;

                Model::CreateCommand(CreateCommandModel::new(CreateCommandModelParameters {
                    name,
                    program,
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
            Router::DeleteWorkspace => {
                self.organizer.delete_workspace(0)?;
                let workspaces = self.organizer.list_workspaces();

                Model::ListWorkspaces(ListWorkspacesModel::new(ListWorkspacesModelParameters {
                    workspaces,
                    search_query: String::new(),
                }))
            }
            Router::DeleteCommand => {
                self.organizer.delete_command(0, 0)?;
                let workspace = self.organizer.get_workspace(0)?;

                Model::GetWorkspace(GetWorkspaceModel::new(GetWorkspaceModelParameters {
                    workspace,
                    commands_search_query: String::new(),
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
            Router::GetCommand(parameters) => {
                let GetCommandParameters { number } = parameters;

                let command = self.organizer.get_command(0, number)?;

                self.organizer.promote_command(0, command.number)?;

                Model::GetCommand(GetCommandModel::new(GetCommandModelParameters { command }))
            }
        };

        self.model = model;
        self.route = route;

        Ok(&mut self.model)
    }

    pub fn new(parameters: AppParameters) -> Result<Self> {
        let AppParameters { organizer } = parameters;
        let workspaces = organizer.list_workspaces();

        let (route, model) = if workspaces.is_empty() {
            let route = Router::ListWorkspaces(ListWorkspacesParameters::default());
            let model = ListWorkspacesModel::new(ListWorkspacesModelParameters {
                workspaces,
                search_query: String::new(),
            });

            (route, Model::ListWorkspaces(model))
        } else {
            let workspace = organizer.get_workspace(0)?;
            let route = Router::GetWorkspace(GetWorkspaceParameters {
                number: 0,
                commands_search_query: String::new(),
            });
            let model = GetWorkspaceModel::new(GetWorkspaceModelParameters {
                workspace,
                commands_search_query: String::new(),
            });

            (route, Model::GetWorkspace(model))
        };

        Ok(Self {
            model,
            route,
            organizer,
        })
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
