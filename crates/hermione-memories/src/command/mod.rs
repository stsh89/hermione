mod name;
mod program;

pub use name::Name;
pub use program::Program;

use crate::Number;

pub struct Command {
    pub(crate) number: Number,
    pub(crate) name: Name,
    pub(crate) program: Program,
}

impl Command {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn number(&self) -> Number {
        self.number
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
