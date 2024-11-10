use crate::{
    coordinator::{Coordinator, ListWorkspacesInput},
    models::{
        ListBackupCredentialsModel, ListBackupCredentialsModelParameters,
        ManageNotionBackupCredentialsModel, ManageNotionBackupCredentialsModelParameters,
    },
    themes::Theme,
    BackupCredentialsPresenter, BackupCredentialsRoute, CommandsHandler,
    DeleteBackupCredentialsParams, ExportParams, ImportParams, ListWorkspaceCommandsParams,
    ListWorkspacesParams, Message, NotionBackupCredentialsPresenter, PowerShellHandler,
    PowerShellRoute, Result, Route, SaveNotionBackupCredentialsParams, WorkspaceCommandsRoute,
    WorkspacesHandler, WorkspacesRoute,
};
use hermione_tui::{BoxedModel, Router};

pub struct TerminalRouter {
    pub coordinator: Coordinator,
    pub theme: Theme,
}

impl Router for TerminalRouter {
    type Route = Route;
    type Message = Message;

    fn default_model(&self) -> Result<BoxedModel<Route, Message>> {
        let workspaces = self.coordinator.list_workspaces(ListWorkspacesInput {
            name_contains: "",
            page_number: None,
            page_size: None,
        })?;

        let handler = WorkspacesHandler {
            coordinator: &self.coordinator,
            theme: self.theme,
        };

        let Some(workspace) = workspaces.into_iter().next() else {
            let model = handler.new_workspace()?;

            return Ok(Box::new(model));
        };

        let handler = CommandsHandler {
            coordinator: &self.coordinator,
            theme: self.theme,
        };

        let model = handler.list(ListWorkspaceCommandsParams {
            workspace_id: workspace.id,
            page_number: None,
            page_size: None,
            search_query: "".into(),
            powershell_no_exit: false,
        })?;

        Ok(Box::new(model))
    }

    fn handle(&self, route: Route) -> Result<Option<BoxedModel<Route, Message>>> {
        match route {
            Route::BackupCredentials(route) => self.handle_backup_credentials_route(route),
            Route::Powershell(route) => self.handle_powershell_route(route),
            Route::Workspaces(route) => self.handle_workspaces_route(route),
        }
    }
}

impl TerminalRouter {
    fn handle_backup_credentials_route(
        &self,
        route: BackupCredentialsRoute,
    ) -> Result<Option<BoxedModel<Route, Message>>> {
        match route {
            BackupCredentialsRoute::List => {
                let backup_credentials_kinds = self.coordinator.list_backup_credentials()?;

                let model = ListBackupCredentialsModel::new(ListBackupCredentialsModelParameters {
                    backup_credentials_kinds,
                    theme: self.theme,
                });

                Ok(Some(Box::new(model)))
            }
            BackupCredentialsRoute::ManageNotionBackupCredentials => {
                let credentials = self.coordinator.find_notion_backup_credentials()?;

                let model = ManageNotionBackupCredentialsModel::new(
                    ManageNotionBackupCredentialsModelParameters {
                        theme: self.theme,
                        credentials,
                        error_message: None,
                    },
                );

                Ok(Some(Box::new(model)))
            }
            BackupCredentialsRoute::SaveNotionBackupCredentials(parameters) => {
                let SaveNotionBackupCredentialsParams {
                    api_key,
                    commands_database_id,
                    workspaces_database_id,
                } = parameters;

                match self.coordinator
                    .save_backup_credentials(BackupCredentialsPresenter::Notion(
                        NotionBackupCredentialsPresenter {
                            api_key: api_key.clone(),
                            commands_database_id: commands_database_id.clone(),
                            workspaces_database_id: workspaces_database_id.clone(),
                        },
                    )) {
                        Ok(()) => {
                            let backup_credentials_kinds = self.coordinator.list_backup_credentials()?;

                            let model = ListBackupCredentialsModel::new(ListBackupCredentialsModelParameters {
                                backup_credentials_kinds,
                                theme: self.theme,
                            });

                            Ok(Some(Box::new(model)))
                        },
                        Err(error) => {
                            let model = ManageNotionBackupCredentialsModel::new(
                                ManageNotionBackupCredentialsModelParameters {
                                    theme: self.theme,
                                    credentials: Some(NotionBackupCredentialsPresenter {
                                        api_key,
                                        workspaces_database_id,
                                        commands_database_id,
                                    }),
                                    error_message: Some(error.to_string()),
                                },
                            );

                            Ok(Some(Box::new(model)))
                        }
                    }
            }
            BackupCredentialsRoute::DeleteBackupCredentials(params) => {
                let DeleteBackupCredentialsParams { kind } = params;

                self.coordinator.delete_backup_credentials(kind)?;

                let backup_credentials_kinds = self.coordinator.list_backup_credentials()?;

                let model = ListBackupCredentialsModel::new(ListBackupCredentialsModelParameters {
                    backup_credentials_kinds,
                    theme: self.theme,
                });

                Ok(Some(Box::new(model)))
            }
            BackupCredentialsRoute::Import(params) => {
                let ImportParams { kind } = params;

                self.coordinator.import(kind)?;

                Ok(None)
            }
            BackupCredentialsRoute::Export(params) => {
                let ExportParams { kind } = params;

                self.coordinator.export(kind)?;

                Ok(None)
            }
        }
    }

    pub fn handle_powershell_route(
        &self,
        route: PowerShellRoute,
    ) -> Result<Option<BoxedModel<Route, Message>>> {
        let handler = PowerShellHandler {
            coordinator: &self.coordinator,
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

    pub fn handle_workspaces_route(
        &self,
        route: WorkspacesRoute,
    ) -> Result<Option<BoxedModel<Route, Message>>> {
        let handler = WorkspacesHandler {
            coordinator: &self.coordinator,
            theme: self.theme,
        };

        match route {
            WorkspacesRoute::Commands(route) => self.handle_workspace_commands_route(route),
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

                let model = handler.list(ListWorkspacesParams {
                    search_query: workspace.name,
                    page_number: None,
                    page_size: None,
                })?;

                Ok(Some(Box::new(model)))
            }
        }
    }
    pub fn handle_workspace_commands_route(
        &self,
        route: WorkspaceCommandsRoute,
    ) -> Result<Option<BoxedModel<Route, Message>>> {
        let handler = CommandsHandler {
            coordinator: &self.coordinator,
            theme: self.theme,
        };

        match route {
            WorkspaceCommandsRoute::Create(paramters) => {
                let command = handler.create(paramters)?;

                let model = handler.list(ListWorkspaceCommandsParams {
                    workspace_id: command.workspace_id,
                    search_query: command.program,
                    page_number: None,
                    page_size: None,
                    powershell_no_exit: false,
                })?;

                Ok(Some(Box::new(model)))
            }
            WorkspaceCommandsRoute::Delete(parameters) => {
                let workspace = handler.delete(parameters)?;

                let model = handler.list(ListWorkspaceCommandsParams {
                    workspace_id: workspace.id,
                    search_query: "".to_string(),
                    page_number: None,
                    page_size: None,
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
