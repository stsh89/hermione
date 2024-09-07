use crate::{Command, Error, Id, Result, Workspace};

pub struct Organizer {
    workspaces: Vec<Workspace>,
}

impl Organizer {
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }

    pub fn add_command(&mut self, workspace_id: &Id, command: Command) -> Result<()> {
        self.get_workspace_mut(workspace_id)?.add_command(command);

        Ok(())
    }

    pub fn delete_command(&mut self, workspace_id: &Id, command_id: &Id) -> Result<()> {
        self.get_workspace_mut(workspace_id)?
            .delete_command(command_id)?;

        Ok(())
    }

    pub fn delete_workspace(&mut self, id: &Id) -> Result<Workspace> {
        if self.workspaces().len() >= id.raw() {
            return Err(Error::workspace_not_found(id));
        }

        Ok(self.workspaces.remove(id.raw()))
    }

    pub fn empty() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }

    pub fn get_command(&self, workspace_id: &Id, command_id: &Id) -> Result<&Command> {
        self.get_workspace(workspace_id)?.get_command(command_id)
    }

    pub fn get_workspace(&self, id: &Id) -> Result<&Workspace> {
        self.workspaces
            .get(id.raw())
            .ok_or(Error::workspace_not_found(id))
    }

    pub fn get_workspace_mut(&mut self, id: &Id) -> Result<&mut Workspace> {
        self.workspaces
            .get_mut(id.raw())
            .ok_or(Error::workspace_not_found(id))
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}
