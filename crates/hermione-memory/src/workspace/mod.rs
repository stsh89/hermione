mod id;
mod name;

use crate::Command;
pub use id::Id;
pub use name::Name;

#[derive(Default)]
pub struct Workspace {
    pub(crate) commands: Vec<Command>,
    pub(crate) id: Id,
    pub(crate) name: Name,
}

impl Workspace {
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
