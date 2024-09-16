use crate::{
    entities::{Command as CommandEntity, Workspace as WorkspaceEntity},
    Result,
};
use hermione_memory::{
    Command, CommandId, LoadOrganizer, NewCommandParameters, NewWorkspaceParameters, Organizer,
    SaveOrganizer, Workspace, WorkspaceId,
};

pub struct Client {
    inner: inner::Client,
    organizer: Organizer,
}

pub struct CreateCommandParameters {
    pub workspace_id: usize,
    pub name: String,
    pub program: String,
}

impl Client {
    pub fn add_command(&mut self, parameters: CreateCommandParameters) -> Result<()> {
        let CreateCommandParameters {
            workspace_id,
            name,
            program,
        } = parameters;

        self.organizer.add_command(
            &WorkspaceId::new(workspace_id),
            NewCommandParameters { name, program },
        )?;

        Ok(())
    }

    pub fn add_workspace(&mut self, name: String) -> Result<WorkspaceEntity> {
        let workspace = self
            .organizer
            .add_workspace(NewWorkspaceParameters { name });

        Ok(from_workspace(workspace))
    }

    pub fn delete_command(&mut self, workspace_id: usize, command_id: usize) -> Result<()> {
        self.organizer
            .delete_command(&WorkspaceId::new(workspace_id), &CommandId::new(command_id))?;

        Ok(())
    }

    pub fn delete_workspace(&mut self, id: usize) -> Result<()> {
        self.organizer.delete_workspace(&WorkspaceId::new(id))?;

        Ok(())
    }

    pub fn get_command(&self, workspace_id: usize, command_id: usize) -> Result<CommandEntity> {
        let command = self
            .organizer
            .get_command(&WorkspaceId::new(workspace_id), &CommandId::new(command_id))?;

        Ok(from_command(command))
    }

    pub fn get_workspace(&self, id: usize) -> Result<WorkspaceEntity> {
        let workspace = self.organizer.get_workspace(&WorkspaceId::new(id))?;

        Ok(from_workspace(workspace))
    }

    pub fn list_workspaces(&self) -> Vec<WorkspaceEntity> {
        self.organizer
            .workspaces()
            .iter()
            .map(from_workspace)
            .collect()
    }

    pub fn new(path: String) -> Result<Self> {
        let inner = inner::Client { path };
        let organizer = LoadOrganizer { loader: &inner }.load()?;
        let client = Self { inner, organizer };

        Ok(client)
    }

    pub fn promote_command(&mut self, workspace_id: usize, command_id: usize) -> Result<()> {
        self.organizer
            .promote_command(&WorkspaceId::new(workspace_id), &CommandId::new(command_id))?;

        Ok(())
    }

    pub fn promote_workspace(&mut self, id: usize) -> Result<()> {
        self.organizer.promote_workspace(&WorkspaceId::new(id))?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        SaveOrganizer { saver: &self.inner }.save(&self.organizer)?;

        Ok(())
    }
}

fn from_workspace(value: &Workspace) -> WorkspaceEntity {
    WorkspaceEntity {
        id: value.id().raw(),
        name: value.name().to_string(),
        commands: value.commands().iter().map(from_command).collect(),
    }
}

fn from_command(value: &Command) -> CommandEntity {
    CommandEntity {
        id: value.id().raw(),
        name: value.name().to_string(),
        program: value.program().to_string(),
    }
}

mod inner {
    use hermione_memory::{
        Command, Error, Load, NewCommandParameters, NewWorkspaceParameters, Organizer, Save,
        Workspace, WorkspaceId,
    };
    use serde::{Deserialize, Serialize};
    use std::{
        fs::{File, OpenOptions},
        io::BufReader,
    };

    pub struct Client {
        pub path: String,
    }

    #[derive(Serialize, Deserialize)]
    struct WorkspaceRecord {
        id: usize,
        name: String,
        commands: Vec<CommandRecord>,
    }

    #[derive(Serialize, Deserialize)]
    struct CommandRecord {
        id: usize,
        name: String,
        program: String,
    }

    impl Load for Client {
        fn load(&self) -> Result<Organizer, Error> {
            let file = File::open(&self.path).map_err(eyre::Report::new)?;
            let reader = BufReader::new(file);
            let workspaces: Vec<WorkspaceRecord> =
                serde_json::from_reader(reader).map_err(eyre::Report::new)?;

            let mut organizer = Organizer::initialize();

            for workspace in workspaces {
                let new_workspace = organizer.add_workspace(NewWorkspaceParameters {
                    name: workspace.name,
                });

                let workspace_id = WorkspaceId::new(new_workspace.id().raw());

                for command in workspace.commands {
                    organizer.add_command(
                        &workspace_id,
                        NewCommandParameters {
                            name: command.name,
                            program: command.program,
                        },
                    )?;
                }
            }

            Ok(organizer)
        }
    }

    impl Save for Client {
        fn save(&self, organizer: &Organizer) -> Result<(), Error> {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .map_err(eyre::Report::new)?;

            let workspaces: Vec<WorkspaceRecord> =
                organizer.workspaces().iter().map(Into::into).collect();

            serde_json::to_writer(&mut file, &workspaces).map_err(eyre::Report::new)?;

            Ok(())
        }
    }

    impl From<&Workspace> for WorkspaceRecord {
        fn from(value: &Workspace) -> Self {
            WorkspaceRecord {
                id: value.id().raw(),
                name: value.name().to_string(),
                commands: value.commands().iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&Command> for CommandRecord {
        fn from(value: &Command) -> Self {
            CommandRecord {
                id: value.id().raw(),
                name: value.name().to_string(),
                program: value.program().to_string(),
            }
        }
    }
}
