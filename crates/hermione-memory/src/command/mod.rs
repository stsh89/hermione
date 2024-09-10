mod id;
mod name;
mod program;

pub use id::Id;
pub use name::Name;
pub use program::Program;

pub struct Command {
    pub(crate) id: Id,
    pub(crate) name: Name,
    pub(crate) program: Program,
}

impl Command {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}
