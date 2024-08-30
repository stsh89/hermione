mod name;

use crate::{Command, Id, OrganizerError, OrganizerResult};
pub use name::Name;

pub struct Workspace {
    commands: Vec<Command>,
    name: Name,
}

impl Workspace {
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub fn get_command(&self, id: &Id) -> OrganizerResult<&Command> {
        self
            .commands
            .get(id.raw())
            .ok_or(OrganizerError::command_not_found(self.name(), id))
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(name: Name) -> Self {
        Self {
            name,
            commands: vec![],
        }
    }

    pub fn remove_command(&mut self, id: &Id) -> OrganizerResult<Command> {
        if self.commands().len() >= id.raw() {
            return Err(OrganizerError::command_not_found(self.name(), id))
        }

        Ok(self.commands.remove(id.raw()))
    }
}
