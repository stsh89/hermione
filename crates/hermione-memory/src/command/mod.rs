mod name;
mod program;

pub use name::Name;
pub use program::Program;

pub struct Command {
    name: Name,
    program: Program,
}

impl Command {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(program: Program) -> Self {
        Self {
            program,
            name: Name::new(String::new()),
        }
    }

    pub fn set_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
