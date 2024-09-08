mod name;
mod program;

pub use name::Name;
pub use program::Program;

pub struct Command {
    name: Name,
    program: Program,
}

pub struct CommandParameters {
    pub name: Name,
    pub program: Program,
}

impl Command {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(parameters: CommandParameters) -> Self {
        let CommandParameters { name, program } = parameters;

        Self { name, program }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
