mod program;
mod name;

use program::Program;
use name::Name;

pub struct Command {
    name: Name,
    program: Program,
}

impl Command {
    pub fn new(program: Program) -> Self {
        Self { program, name: Name::new(String::new())}
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
