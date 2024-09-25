use crate::{
    entities::{Command as CommandEntity, Workspace as WorkspaceEntity},
    Result,
};
use hermione_memory::{
    Command, CommandParameters as HermioneCommandParameters, LoadOrganizer, Location,
    NewWorkspaceParameters, Organizer, SaveOrganizer, Workspace, WorkspaceName,
};

pub struct Client {
    inner: inner::Client,
    organizer: Organizer,
}

pub struct CommandParameters {
    pub workspace_number: usize,
    pub name: String,
    pub program: String,
}

impl Client {
    pub fn add_command(&mut self, parameters: CommandParameters) -> Result<()> {
        let CommandParameters {
            workspace_number,
            name,
            program,
        } = parameters;

        self.organizer.add_command(
            workspace_number.into(),
            HermioneCommandParameters { name, program },
        )?;

        Ok(())
    }

    pub fn add_workspace(&mut self, name: String) -> Result<WorkspaceEntity> {
        let workspace = self
            .organizer
            .add_workspace(NewWorkspaceParameters { name });

        Ok(from_workspace(workspace))
    }

    pub fn delete_command(&mut self, workspace_number: usize, command_number: usize) -> Result<()> {
        self.organizer
            .delete_command(workspace_number.into(), command_number.into())?;

        Ok(())
    }

    pub fn delete_workspace(&mut self, number: usize) -> Result<()> {
        self.organizer.delete_workspace(number.into())?;

        Ok(())
    }

    pub fn get_command(
        &self,
        workspace_number: usize,
        command_number: usize,
    ) -> Result<CommandEntity> {
        let command = self
            .organizer
            .get_command(workspace_number.into(), command_number.into())?;

        Ok(from_command(command))
    }

    pub fn get_workspace(&self, number: usize) -> Result<WorkspaceEntity> {
        let workspace = self.organizer.get_workspace(number.into())?;

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

    pub fn promote_command(
        &mut self,
        workspace_number: usize,
        command_number: usize,
    ) -> Result<()> {
        self.organizer
            .promote_command(workspace_number.into(), command_number.into())?;

        Ok(())
    }

    pub fn promote_workspace(&mut self, number: usize) -> Result<()> {
        self.organizer.promote_workspace(number.into())?;

        Ok(())
    }

    pub fn rename_workspace(&mut self, number: usize, name: String) -> Result<()> {
        self.organizer
            .rename_workspace(number.into(), WorkspaceName::new(name))?;

        Ok(())
    }

    pub fn set_workspace_location(&mut self, number: usize, location: String) -> Result<()> {
        self.organizer
            .set_workspace_location(number.into(), Location::new(location))?;

        Ok(())
    }

    pub fn update_command(
        &mut self,
        command_number: usize,
        parameters: CommandParameters,
    ) -> Result<()> {
        let CommandParameters {
            workspace_number,
            name,
            program,
        } = parameters;

        self.organizer.update_command(
            workspace_number.into(),
            command_number.into(),
            HermioneCommandParameters { name, program },
        )?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        SaveOrganizer { saver: &self.inner }.save(&self.organizer)?;

        Ok(())
    }
}

fn from_workspace(value: &Workspace) -> WorkspaceEntity {
    WorkspaceEntity {
        number: value.number().into(),
        name: value.name().to_string(),
        commands: value.commands().iter().map(from_command).collect(),
        location: value
            .location()
            .map(ToString::to_string)
            .unwrap_or_default(),
    }
}

fn from_command(value: &Command) -> CommandEntity {
    CommandEntity {
        number: value.number().into(),
        name: value.name().to_string(),
        program: value.program().to_string(),
    }
}

mod inner {
    use hermione_memory::{
        Command, CommandParameters, Error, Load, NewWorkspaceParameters, Organizer, Save, Workspace,
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
        number: usize,
        name: String,
        commands: Vec<CommandRecord>,

        #[serde(skip_serializing_if = "Option::is_none")]
        location: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    struct CommandRecord {
        number: usize,
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
                let number = organizer
                    .add_workspace(NewWorkspaceParameters {
                        name: workspace.name,
                    })
                    .number();

                for command in workspace.commands {
                    organizer.add_command(
                        number,
                        CommandParameters {
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
                number: value.number().into(),
                name: value.name().to_string(),
                commands: value.commands().iter().map(Into::into).collect(),
                location: value.location().map(ToString::to_string),
            }
        }
    }

    impl From<&Command> for CommandRecord {
        fn from(value: &Command) -> Self {
            CommandRecord {
                number: value.number().into(),
                name: value.name().to_string(),
                program: value.program().to_string(),
            }
        }
    }
}
