use crate::{data::Command as DataCommand, data::Workspace as DataWorkspace, Result};
use hermione_memory::{
    Command, CommandName, Error, Id, Load, LoadOrganizer, Organizer, Program, Save, SaveOrganizer,
    Workspace, WorkspaceName,
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
        let mut command = Command::new(Program::new(program));
        command.set_name(CommandName::new(name));
        self.organizer
            .add_command(&Id::new(workspace_index), command)?;

        Ok(())
    }

    pub fn delete_command(&mut self, workspace_index: usize, command_index: usize) -> Result<()> {
        self.organizer
            .delete_command(&Id::new(workspace_index), &Id::new(command_index))?;

        Ok(())
    }

    pub fn get_command(&self, workspace_index: usize, command_index: usize) -> Result<DataCommand> {
        let command = self
            .organizer
            .get_command(&Id::new(workspace_index), &Id::new(command_index))?;

        Ok(command.into())
    }

    pub fn get_workspace(&self, index: usize) -> Result<DataWorkspace> {
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
        let workspace = Workspace::new(WorkspaceName::new(name));

        self.organizer.add_workspace(workspace);

        Ok(())
    }

    pub fn delete_workspace(&mut self, index: usize) -> Result<()> {
        self.organizer.delete_workspace(&Id::new(index))?;

        Ok(())
    }

    pub fn workspaces(&self) -> Vec<DataWorkspace> {
        self.organizer.workspaces().iter().map(Into::into).collect()
    }
}

pub struct Inner;

impl Load for Inner {
    fn load(&self) -> Result<Organizer, Error> {
        let mut organizer = Organizer::empty();
        let mut workspace = Workspace::new(WorkspaceName::new("Hermione".to_string()));
        let mut command = Command::new(Program::new("cargo fmt".to_string()));
        command.set_name(CommandName::new("Format project".to_string()));
        workspace.add_command(command);

        let mut command = Command::new(Program::new("cargo clippy".to_string()));
        command.set_name(CommandName::new("Lint project".to_string()));
        workspace.add_command(command);
        organizer.add_workspace(workspace);

        let mut workspace = Workspace::new(WorkspaceName::new("General".to_string()));
        let command = Command::new(Program::new("Get-ChildItem".to_string()));
        workspace.add_command(command);
        organizer.add_workspace(workspace);

        let mut workspace = Workspace::new(WorkspaceName::new("Vulkan tutorial".to_string()));
        let mut command = Command::new(Program::new(
            r#"C:\VulkanSDK\1.3.290.0\Bin\glslc.exe .\shaders\shader.frag -o .\shaders\frag.spv"#
                .to_string(),
        ));
        command.set_name(CommandName::new("Compile shader fragment".to_string()));
        workspace.add_command(command);
        organizer.add_workspace(workspace);
        Ok(organizer)
    }
}

impl Save for Inner {
    fn save(&self, _organizer: &Organizer) -> Result<(), Error> {
        Ok(())
    }
}

impl From<&Workspace> for crate::data::Workspace {
    fn from(value: &Workspace) -> Self {
        crate::data::Workspace {
            name: value.name().to_string(),
            commands: value.commands().iter().map(Into::into).collect(),
        }
    }
}

impl From<&Command> for crate::data::Command {
    fn from(value: &Command) -> Self {
        crate::data::Command {
            name: value.name().to_string(),
            program: value.program().to_string(),
        }
    }
}
