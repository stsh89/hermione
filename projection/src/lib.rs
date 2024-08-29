mod instruction;
mod workspace;
mod operation;
mod projection_error;

pub use instruction::{Instruction, InstructionAttributes};
pub use workspace::Workspace;
pub use projection_error::ProjectionError;
pub use operation::{LoadProjection, Load, SaveProjection, Save};


#[derive(Default)]
pub struct Projection {
    workspaces: Vec<Workspace>,
}

impl Projection {
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }

    pub fn get_instruction(
        &self,
        workspace_index: usize,
        instruction_index: usize,
    ) -> Option<&Instruction> {
        self.get_workspace(workspace_index)
            .and_then(|w| w.get_instruction(instruction_index))
    }

    pub fn get_workspace(&self, index: usize) -> Option<&Workspace> {
        self.workspaces.get(index)
    }

    pub fn get_workspace_mut(&mut self, index: usize) -> Option<&mut Workspace> {
        self.workspaces.get_mut(index)
    }

    pub fn remove_workspace(&mut self, index: usize) {
        self.workspaces.remove(index);
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}
