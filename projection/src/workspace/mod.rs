use crate::Instruction;

pub struct Workspace {
    instructions: Vec<Instruction>,
    name: String,
}

pub struct WorkspaceAttributes {
    pub instructions: Vec<Instruction>,
    pub name: String,
}

impl Workspace {
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn new(attributes: WorkspaceAttributes) -> Self {
        let WorkspaceAttributes { instructions, name } = attributes;

        Self { name, instructions }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn remove_instruction(&mut self, index: usize) {
        self.instructions.remove(index);
    }
}
