use crate::{
    clients::{CreateCommandParameters, OrganizerClient},
    models::{
        command_center::{
            Model as CommandCenterModel, ModelParameters as CommandCenterModelParameters,
            NewCommand,
        },
        command_display::{
            Model as CommandDisplayModel, ModelParameters as CommandDisplayModelParameters,
        },
        lobby::{Model as LobbyModel, ModelParameters as LobbyModelParameters},
        new_command::Model as NewCommandModel,
        new_workspace::Model as NewWorkspaceModel,
    },
    runners::{
        command_center::{
            Runner as CommandCenterRunner, RunnerParameters as CommandCenterRunnerParameters,
            Signal as CommandCenterSignal,
        },
        command_display::{
            Runner as CommandDisplayRunner, RunnerParameters as CommandDisplayRunnerParameters,
        },
        lobby::{
            Runner as LobbyRunner, RunnerParameters as LobbyRunnerParameters, Signal as LobbySignal,
        },
        new_command::{Runner as NewCommandRunner, RunnerParameters as NewCommandRunnerParameters},
        new_workspace::{
            Runner as NewWorkspaceRunner, RunnerParameters as NewWorkspaceRunnerParameters,
        },
    },
    Result,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct Lobby<'a> {
    pub organizer: OrganizerClient,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> Lobby<'a> {
    pub fn enter(mut self) -> Result<()> {
        loop {
            let runner = LobbyRunner::new(LobbyRunnerParameters {
                terminal: self.terminal,
                model: LobbyModel::new(LobbyModelParameters {
                    organizer: &mut self.organizer,
                })?,
            });

            match runner.run()? {
                LobbySignal::EnterCommandCenter(workspace_id) => {
                    self.enter_command_center(workspace_id)?
                }
                LobbySignal::NewWorkspaceRequest => self.new_workspace()?,
                LobbySignal::Exit => break,
            };
        }

        Ok(())
    }

    fn enter_command_center(&mut self, workspace_id: usize) -> Result<()> {
        CommandCenter {
            workspace_id,
            organizer: &mut self.organizer,
            terminal: self.terminal,
        }
        .enter()
    }

    fn new_workspace(&mut self) -> Result<()> {
        let runner = NewWorkspaceRunner::new(NewWorkspaceRunnerParameters {
            terminal: self.terminal,
            model: NewWorkspaceModel::new(),
        });

        if let Some(name) = runner.run()? {
            self.organizer.add_workspace(name)?;
        }

        Ok(())
    }
}

struct CommandCenter<'a> {
    pub workspace_id: usize,
    pub organizer: &'a mut OrganizerClient,
    pub terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> CommandCenter<'a> {
    fn execute_command(&mut self, command_id: usize) -> Result<()> {
        let command = self.organizer.get_command(self.workspace_id, command_id)?;
        let runner = CommandDisplayRunner::new(CommandDisplayRunnerParameters {
            terminal: self.terminal,
            model: CommandDisplayModel::new(CommandDisplayModelParameters { command })?,
        });

        runner.run()
    }

    fn enter(mut self) -> Result<()> {
        loop {
            let workspace = self.organizer.get_workspace(self.workspace_id)?;

            let runner = CommandCenterRunner::new(CommandCenterRunnerParameters {
                terminal: self.terminal,
                model: CommandCenterModel::new(CommandCenterModelParameters {
                    organizer: self.organizer,
                    workspace,
                })?,
            });

            match runner.run()? {
                CommandCenterSignal::ExecuteCommand(command_id) => {
                    self.execute_command(command_id)?
                }
                CommandCenterSignal::NewCommandRequest => self.new_command()?,
                CommandCenterSignal::Exit => break,
            };
        }

        Ok(())
    }

    fn new_command(&mut self) -> Result<()> {
        let runner = NewCommandRunner::new(NewCommandRunnerParameters {
            terminal: self.terminal,
            model: NewCommandModel::new(),
        });

        if let Some(new_command) = runner.run()? {
            let NewCommand { name, program } = new_command;

            self.organizer.add_command(CreateCommandParameters {
                workspace_id: self.workspace_id,
                name,
                program,
            })?;
        }

        Ok(())
    }
}
