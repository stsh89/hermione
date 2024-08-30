use crate::{Command, Id, OrganizerError, OrganizerResult, Workspace};

pub struct Organizer {
    workspaces: Vec<Workspace>,
}

impl Organizer {
    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.push(workspace);
    }

    pub fn empty() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }

    pub fn get_command(&self, workspace_id: &Id, command_id: &Id) -> OrganizerResult<&Command> {
        self.get_workspace(workspace_id)?.get_command(command_id)
    }

    pub fn get_workspace(&self, id: &Id) -> OrganizerResult<&Workspace> {
        self.workspaces
            .get(id.raw())
            .ok_or(OrganizerError::workspace_not_found(id))
    }

    pub fn get_workspace_mut(&mut self, id: &Id) -> OrganizerResult<&mut Workspace> {
        self.workspaces
            .get_mut(id.raw())
            .ok_or(OrganizerError::workspace_not_found(id))
    }

    pub fn remove_workspace(&mut self, id: &Id) -> OrganizerResult<Workspace> {
        if self.workspaces().len() >= id.raw() {
            return Err(OrganizerError::workspace_not_found(id));
        }

        Ok(self.workspaces.remove(id.raw()))
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}
