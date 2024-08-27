use crate::Message;
use projection::{Instruction, InstructionAttributes, Projection, Workspace};

pub struct Model {
    projection: Projection,
    state: State,
}

pub enum State {
    Initialized,
    WorkspacePreview(usize),
    Exited,
}

impl Model {
    fn exit(&mut self) {
        self.state = State::Exited;
    }

    pub fn initialize() -> Self {
        let mut projection = Projection::default();
        let mut workspace = Workspace::new("Hermione".to_string());
        workspace.add_instruction(Instruction::new(InstructionAttributes {
            name: "Format project".to_string(),
            directive: "cargo fmt".to_string(),
        }));
        workspace.add_instruction(Instruction::new(InstructionAttributes {
            name: "Lint project".to_string(),
            directive: "cargo clippy".to_string(),
        }));
        projection.add_workspace(workspace);
        projection.add_workspace(Workspace::new("General".to_string()));

        let mut workspace = Workspace::new("Vulkan tutorial".to_string());
        workspace.add_instruction(Instruction::new(InstructionAttributes { name: "Compile shader fragment".to_string(), directive: r#"C:\VulkanSDK\1.3.290.0\Bin\glslc.exe .\shaders\shader.frag -o .\shaders\frag.spv"#.to_string() }));
        projection.add_workspace(workspace);

        Self {
            state: State::Initialized,
            projection,
        }
    }

    pub fn is_exited(&self) -> bool {
        matches!(self.state, State::Exited)
    }

    fn preview_next_workspace(&mut self) {
        if self.projection.workspaces().is_empty() {
            return;
        }

        match self.state {
            State::Initialized => self.state = State::WorkspacePreview(0),
            State::WorkspacePreview(workspace_index) => {
                let index_reset = 0;
                let next_index = workspace_index + 1;

                let index = if next_index == self.projection.workspaces().len() {
                    index_reset
                } else {
                    next_index
                };

                self.state = State::WorkspacePreview(index);
            }
            State::Exited => todo!(),
        }
    }

    pub fn preview_workspace_commands(&self) -> Vec<&str> {
        match self.state {
            State::Initialized => vec![],
            State::WorkspacePreview(index) => {
                if let Some(workspace) = self.projection.workspaces().get(index) {
                    workspace
                        .instructions()
                        .iter()
                        .map(|i| i.directive())
                        .collect()
                } else {
                    vec![]
                }
            }
            State::Exited => vec![],
        }
    }

    fn preview_previous_workspace(&mut self) {
        if self.projection.workspaces().is_empty() {
            return;
        }

        match self.state {
            State::Initialized => {
                self.state = State::WorkspacePreview(self.projection.workspaces().len() - 1)
            }
            State::WorkspacePreview(workspace_index) => {
                let index = if workspace_index == 0 {
                    self.projection.workspaces().len() - 1
                } else {
                    workspace_index - 1
                };

                self.state = State::WorkspacePreview(index);
            }
            State::Exited => todo!(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddWorkspace => todo!(),
            Message::EnterWorkspace => todo!(),
            Message::Exit => self.exit(),
            Message::ExitWorkspace => todo!(),
            Message::NewWorkspace => todo!(),
            Message::RemoveWorkspace => todo!(),
            Message::PreviewNextWorkspace => self.preview_next_workspace(),
            Message::PreviewPreviousWorkspace => self.preview_previous_workspace(),
        }
    }

    pub fn workspace_names(&self) -> Vec<&str> {
        self.projection
            .workspaces()
            .iter()
            .map(|w| w.name())
            .collect()
    }

    pub fn current_workspace_preview_index(&self) -> Option<usize> {
        match self.state {
            State::Initialized => None,
            State::WorkspacePreview(index) => Some(index),
            State::Exited => None,
        }
    }
}
