use handbag::{Command, InstructionAttributes, Organizer, Workspace};

use crate::lens::Lens;

pub struct App {}

impl App {
    pub fn run(&self) -> std::io::Result<()> {
        let mut terminal = ratatui::init();
        let mut lens = Lens::open(organizer());

        loop {
            terminal.draw(|frame| lens.view(frame))?;

            let message = lens.handle_event()?;

            if let Some(message) = message {
                lens.update(message);
            }

            if lens.is_closed() {
                break;
            }
        }
        ratatui::restore();

        Ok(())
    }
}

fn organizer() -> Organizer {
    let mut organizer = Organizer::default();
    let mut workspace = Workspace::new("Hermione".to_string());
    workspace.add_command(Command::new(InstructionAttributes {
        name: "Format project".to_string(),
        directive: "cargo fmt".to_string(),
    }));
    workspace.add_command(Command::new(InstructionAttributes {
        name: "Lint project".to_string(),
        directive: "cargo clippy".to_string(),
    }));
    organizer.add_workspace(workspace);

    let mut workspace = Workspace::new("General".to_string());
    workspace.add_command(Command::new(InstructionAttributes {
        name: "".to_string(),
        directive: "Get-ChildItem".to_string(),
    }));
    organizer.add_workspace(workspace);

    let mut workspace = Workspace::new("Vulkan tutorial".to_string());
    workspace.add_command(Command::new(InstructionAttributes {
        name: "Compile shader fragment".to_string(),
        directive:
            r#"C:\VulkanSDK\1.3.290.0\Bin\glslc.exe .\shaders\shader.frag -o .\shaders\frag.spv"#
                .to_string(),
    }));
    organizer.add_workspace(workspace);
    organizer
}
