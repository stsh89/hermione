mod instruction;
mod workspace;

pub use instruction::Instruction;
pub use workspace::Workspace;

#[derive(Default)]
pub struct Projection {
    workspaces: Vec<Workspace>,
}

impl Projection {
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
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
