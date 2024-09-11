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

pub struct App {
    pub organizer: OrganizerClient,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn run(mut self) -> Result<()> {
        loop {
            let runner = LobbyRunner::new(LobbyRunnerParameters {
                terminal: &mut self.terminal,
                model: LobbyModel::new(LobbyModelParameters {
                    organizer: &mut self.organizer,
                })?,
            });

            let signal = runner.run()?;

            match signal {
                LobbySignal::EnterCommandCenter(workspace_id) => loop {
                    let workspace = self.organizer.get_workspace(workspace_id)?;
                    let runner = CommandCenterRunner::new(CommandCenterRunnerParameters {
                        terminal: &mut self.terminal,
                        model: CommandCenterModel::new(CommandCenterModelParameters {
                            organizer: &mut self.organizer,
                            workspace,
                        })?,
                    });

                    let signal = runner.run()?;

                    match signal {
                        CommandCenterSignal::ExecuteCommand(command_id) => {
                            let command = self.organizer.get_command(workspace_id, command_id)?;
                            let runner =
                                CommandDisplayRunner::new(CommandDisplayRunnerParameters {
                                    terminal: &mut self.terminal,
                                    model: CommandDisplayModel::new(
                                        CommandDisplayModelParameters { command },
                                    )?,
                                });

                            runner.run()?;
                        }
                        CommandCenterSignal::NewCommandRequest(workspace_id) => {
                            let runner = NewCommandRunner::new(NewCommandRunnerParameters {
                                terminal: &mut self.terminal,
                                model: NewCommandModel::new(),
                            });

                            if let Some(new_command) = runner.run()? {
                                let NewCommand { name, program } = new_command;

                                self.organizer.add_command(CreateCommandParameters {
                                    workspace_id,
                                    name,
                                    program,
                                })?;
                            }
                        }
                        CommandCenterSignal::Exit => break,
                    }
                },
                LobbySignal::NewWorkspaceRequest => {
                    let runner = NewWorkspaceRunner::new(NewWorkspaceRunnerParameters {
                        terminal: &mut self.terminal,
                        model: NewWorkspaceModel::new(),
                    });

                    if let Some(name) = runner.run()? {
                        self.organizer.add_workspace(name)?;
                    }
                }
                LobbySignal::Exit => return Ok(()),
            }
        }
    }
}
