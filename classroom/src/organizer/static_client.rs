use handbag::{
    Command, CommandName, Load, LoadOrganizer, Organizer, Program, Workspace, WorkspaceName,
};

use crate::AppResult;

pub struct StaticClient;

impl Load for StaticClient {
    fn load(&self) -> Result<handbag::Organizer, handbag::OrganizerError> {
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

impl StaticClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_organizer(&self) -> AppResult<Organizer> {
        let organizer = self.load()?;

        Ok(organizer)
    }
}
