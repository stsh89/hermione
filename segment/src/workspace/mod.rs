mod name;

use crate::Instruction;
use name::Name;

pub struct Workspace {
    name: Name,
    instructions: Vec<Instruction>,
}

pub struct WorkspaceAttributes {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl Workspace {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(attributes: WorkspaceAttributes) -> Self {
        let WorkspaceAttributes { name, instructions } = attributes;

        Self {
            name: Name::new(name),
            instructions,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
