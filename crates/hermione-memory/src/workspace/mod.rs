mod location;
mod name;

use crate::{Command, Number};
pub use location::Location;
pub use name::Name;

pub struct Workspace {
    pub(crate) commands: Vec<Command>,
    pub(crate) name: Name,
    pub(crate) number: Number,
    pub(crate) location: Option<Location>,
}

impl Workspace {
    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
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
