use crate::{
    CommandsHandler, Coordinator, ListWorkspaceCommandsParameters, ListWorkspacesFilter,
    ListWorkspacesParameters, Message, PowerShellHandler, PowerShellRoute, Result, Route,
    WorkspaceCommandsRoute, WorkspacesHandler, WorkspacesRoute, LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
};
use hermione_powershell::PowerShell;
use hermione_tui::{BoxedModel, Router};

pub struct TerminalRouter {
    pub coordinator: Coordinator,
    pub powershell: PowerShell,
}

struct WorkspacesRouter<'a> {
    coordinator: &'a Coordinator,
}

struct WorkspaceCommandsRouter<'a> {
    coordinator: &'a Coordinator,
}

struct PowerShellRouter<'a> {
    coordinator: &'a Coordinator,
    powershell: &'a PowerShell,
}

impl Router for TerminalRouter {
    type Route = Route;
    type Message = Message;

    fn default_model(&self) -> Result<BoxedModel<Route, Message>> {
        let workspaces = self.coordinator.workspaces().list(ListWorkspacesFilter {
            page_number: 0,
            page_size: 1,
            name_contains: "",
        })?;

        let handler = WorkspacesHandler {
            coordinator: &self.coordinator,
        };

        let Some(workspace) = workspaces.into_iter().next() else {
            let model = handler.new_workspace()?;

            return Ok(Box::new(model));
        };

        let handler = CommandsHandler {
            coordinator: &self.coordinator,
        };

        let model = handler.list(ListWorkspaceCommandsParameters {
            workspace_id: workspace.id,
            page_number: 0,
            page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
            search_query: "".into(),
            powershell_no_exit: false,
        })?;

        Ok(Box::new(model))
    }

    fn handle(&self, route: Route) -> Result<Option<BoxedModel<Route, Message>>> {
        let TerminalRouter {
            coordinator,
            powershell,
        } = self;

        match route {
            Route::Workspaces(route) => WorkspacesRouter { coordinator }.handle(route),
            Route::Powershell(route) => PowerShellRouter {
                coordinator,
                powershell,
            }
            .handle(route),
        }
    }
}

impl<'a> PowerShellRouter<'a> {
    pub fn handle(self, route: PowerShellRoute) -> Result<Option<BoxedModel<Route, Message>>> {
        let PowerShellRouter {
            coordinator,
            powershell,
        } = self;

        let handler = PowerShellHandler {
            coordinator,
            powershell,
        };

        match route {
            PowerShellRoute::CopyToClipboard(parameters) => {
                handler.copy_to_clipboard(parameters)?;

                Ok(None)
            }
            PowerShellRoute::ExecuteCommand(parameters) => {
                handler.execute_command(parameters)?;

                Ok(None)
            }
            PowerShellRoute::OpenWindowsTerminal(parameters) => {
                handler.open_windows_terminal(parameters)?;

                Ok(None)
            }
        }
    }
}

impl<'a> WorkspacesRouter<'a> {
    pub fn handle(self, route: WorkspacesRoute) -> Result<Option<BoxedModel<Route, Message>>> {
        let WorkspacesRouter { coordinator } = self;
        let handler = WorkspacesHandler { coordinator };

        match route {
            WorkspacesRoute::Commands(route) => {
                WorkspaceCommandsRouter { coordinator }.handle(route)
            }
            WorkspacesRoute::Create(parameters) => {
                let model = handler.create(parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspacesRoute::Delete(parameters) => {
                let model = handler.delete(parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspacesRoute::Edit(paramters) => {
                let model = handler.edit(paramters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspacesRoute::List(list_parameters) => {
                let model = handler.list(list_parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspacesRoute::New => {
                let model = handler.new_workspace()?;

                Ok(Some(Box::new(model)))
            }
            WorkspacesRoute::Update(parameters) => {
                let workspace = handler.update(parameters)?;
                let model = handler.list(ListWorkspacesParameters {
                    search_query: workspace.name,
                    page_number: 0,
                    page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                })?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}

impl<'a> WorkspaceCommandsRouter<'a> {
    pub fn handle(
        self,
        route: WorkspaceCommandsRoute,
    ) -> Result<Option<BoxedModel<Route, Message>>> {
        let WorkspaceCommandsRouter { coordinator } = self;
        let handler = CommandsHandler { coordinator };

        match route {
            WorkspaceCommandsRoute::Create(paramters) => {
                let command = handler.create(paramters)?;

                let model = handler.list(ListWorkspaceCommandsParameters {
                    workspace_id: command.workspace_id,
                    search_query: command.program,
                    page_number: 0,
                    page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                    powershell_no_exit: false,
                })?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::Delete(parameters) => {
                let workspace = handler.delete(parameters)?;

                let model = handler.list(ListWorkspaceCommandsParameters {
                    workspace_id: workspace.id,
                    search_query: "".to_string(),
                    page_number: 0,
                    page_size: LIST_WORKSPACE_COMMANDS_PAGE_SIZE,
                    powershell_no_exit: false,
                })?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::Edit(parameters) => {
                let model = handler.edit(parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::List(parameters) => {
                let model = handler.list(parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::New(parameters) => {
                let model = handler.new_command(parameters)?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::Update(parameters) => {
                let model = handler.update(parameters)?;

                Ok(Some(Box::new(model)))
            }
        }
    }
}
