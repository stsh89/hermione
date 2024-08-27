use crate::Instruction;

pub struct Workspace {
    instructions: Vec<Instruction>,
    name: String,
}

impl Workspace {
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn remove_instruction(&mut self, index: usize) {
        self.instructions.remove(index);
    }
}
