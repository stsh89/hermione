use crate::Instruction;

pub struct Workspace {
    instructions: Vec<Instruction>,
    name: String,
}

impl Workspace {
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn get_instruction(&self, index: usize) -> Option<&Instruction> {
        self.instructions.get(index)
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            instructions: vec![],
        }
    }

    pub fn remove_instruction(&mut self, index: usize) {
        self.instructions.remove(index);
    }
}
