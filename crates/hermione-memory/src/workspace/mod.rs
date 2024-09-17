mod name;

use crate::{Command, Number};
pub use name::Name;

pub struct Workspace {
    pub(crate) commands: Vec<Command>,
    pub(crate) name: Name,
    pub(crate) number: Number,
}

impl Workspace {
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn number(&self) -> Number {
        self.number
    }

    pub(crate) fn update_command_numbers(&mut self) {
        self.commands
            .iter_mut()
            .enumerate()
            .for_each(|(index, command)| {
                command.number = index.into();
            });
    }
}
