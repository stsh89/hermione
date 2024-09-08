mod name;

use crate::{Command, Error, Id, Result};
pub use name::Name;

pub struct Workspace {
    commands: Vec<Command>,
    name: Name,
}

pub struct WorkspaceParameters {
    pub name: Name,
    pub commands: Vec<Command>,
}

impl Workspace {
    pub(crate) fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    pub(crate) fn delete_command(&mut self, id: &Id) -> Result<Command> {
        if self.commands().len() <= id.raw() {
            return Err(Error::command_not_found(self.name(), id));
        }

        Ok(self.commands.remove(id.raw()))
    }

    pub fn get_command(&self, id: &Id) -> Result<&Command> {
        self.commands
            .get(id.raw())
            .ok_or(Error::command_not_found(self.name(), id))
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(parameters: WorkspaceParameters) -> Self {
        let WorkspaceParameters { name, commands } = parameters;

        Self { name, commands }
    }
}
