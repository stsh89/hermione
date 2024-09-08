use std::{
    fs::{File, OpenOptions},
    io::BufReader,
};

use crate::{data::Command as CommandData, data::Workspace as WorkspaceData, Result};
use hermione_memory::{
    Command, CommandName, CommandParameters, Error, Id, Load, LoadOrganizer, Organizer, Program,
    Save, SaveOrganizer, Workspace, WorkspaceName, WorkspaceParameters,
};

pub struct Client {
    inner: Inner,
    organizer: Organizer,
}

impl Client {
    pub fn create_command(
        &mut self,
        workspace_index: usize,
        name: String,
        program: String,
    ) -> Result<()> {
        let command = Command::new(CommandParameters {
            program: Program::new(program),
            name: CommandName::new(name.clone()),
        });

        self.organizer
            .add_command(&Id::new(workspace_index), command)?;

        Ok(())
    }

    pub fn delete_command(&mut self, workspace_index: usize, command_index: usize) -> Result<()> {
        self.organizer
            .delete_command(&Id::new(workspace_index), &Id::new(command_index))?;

        Ok(())
    }

    pub fn get_command(&self, workspace_index: usize, command_index: usize) -> Result<CommandData> {
        let command = self
            .organizer
            .get_command(&Id::new(workspace_index), &Id::new(command_index))?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, index: usize) -> Result<WorkspaceData> {
        let workspace = self.organizer.get_workspace(&Id::new(index))?;

        Ok(workspace.into())
    }

    pub fn new() -> Result<Self> {
        let inner = Inner {};

        let organizer = LoadOrganizer { loader: &inner }.load()?;

        let client = Self { inner, organizer };

        Ok(client)
    }

    pub fn save(&self) -> Result<()> {
        SaveOrganizer { saver: &self.inner }.save(&self.organizer)?;

        Ok(())
    }

    pub fn create_workspace(&mut self, name: String) -> Result<()> {
        let workspace = Workspace::new(WorkspaceParameters {
            name: WorkspaceName::new(name),
            commands: vec![],
        });

        self.organizer.add_workspace(workspace);

        Ok(())
    }

    pub fn delete_workspace(&mut self, index: usize) -> Result<()> {
        self.organizer.delete_workspace(&Id::new(index))?;

        Ok(())
    }

    pub fn workspaces(&self) -> Vec<WorkspaceData> {
        self.organizer.workspaces().iter().map(Into::into).collect()
    }
}

pub struct Inner;

impl Load for Inner {
    fn load(&self) -> Result<Organizer, Error> {
        let file = File::open("hermione.json").map_err(eyre::Report::new)?;
        let reader = BufReader::new(file);
        let workspaces: Vec<WorkspaceData> =
            serde_json::from_reader(reader).map_err(eyre::Report::new)?;

        let mut organizer = Organizer::empty();

        workspaces.into_iter().for_each(|workspace| {
            organizer.add_workspace(workspace.into());
        });

        Ok(organizer)

        // let mut organizer = Organizer::empty();
        // let mut workspace = Workspace::new(WorkspaceName::new("Hermione".to_string()));
        // let mut command = Command::new(Program::new("cargo fmt".to_string()));
        // command.set_name(CommandName::new("Format project".to_string()));
        // workspace.add_command(command);

        // let mut command = Command::new(Program::new("cargo clippy".to_string()));
        // command.set_name(CommandName::new("Lint project".to_string()));
        // workspace.add_command(command);
        // organizer.add_workspace(workspace);

        // let mut workspace = Workspace::new(WorkspaceName::new("General".to_string()));
        // let command = Command::new(Program::new("Get-ChildItem".to_string()));
        // workspace.add_command(command);
        // organizer.add_workspace(workspace);

        // let mut workspace = Workspace::new(WorkspaceName::new("Vulkan tutorial".to_string()));
        // let mut command = Command::new(Program::new(
        //     r#"C:\VulkanSDK\1.3.290.0\Bin\glslc.exe .\shaders\shader.frag -o .\shaders\frag.spv"#
        //         .to_string(),
        // ));
        // command.set_name(CommandName::new("Compile shader fragment".to_string()));
        // workspace.add_command(command);
        // organizer.add_workspace(workspace);
        // Ok(organizer)
    }
}

impl Save for Inner {
    fn save(&self, organizer: &Organizer) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .open("hermione.json")
            .map_err(eyre::Report::new)?;

        let workspaces: Vec<WorkspaceData> =
            organizer.workspaces().iter().map(Into::into).collect();

        serde_json::to_writer(&mut file, &workspaces).map_err(eyre::Report::new)?;

        Ok(())
    }
}

impl From<&Workspace> for WorkspaceData {
    fn from(value: &Workspace) -> Self {
        WorkspaceData {
            name: value.name().to_string(),
            commands: value.commands().iter().map(Into::into).collect(),
        }
    }
}

impl From<&Command> for CommandData {
    fn from(value: &Command) -> Self {
        CommandData {
            name: value.name().to_string(),
            program: value.program().to_string(),
        }
    }
}

impl From<WorkspaceData> for Workspace {
    fn from(value: WorkspaceData) -> Self {
        Workspace::new(WorkspaceParameters {
            name: WorkspaceName::new(value.name),
            commands: value.commands.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<CommandData> for Command {
    fn from(value: CommandData) -> Self {
        Command::new(CommandParameters {
            program: Program::new(value.program),
            name: CommandName::new(value.name),
        })
    }
}
