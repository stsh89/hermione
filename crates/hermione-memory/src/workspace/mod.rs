mod id;
mod name;

use crate::{Command, CommandId};
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

    pub(crate) fn update_command_ids(&mut self) {
        self.commands
            .iter_mut()
            .enumerate()
            .for_each(|(index, command)| {
                command.id = CommandId::new(index);
            });
    }
}
