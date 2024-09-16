use crate::{
    Command, CommandId, CommandName, Error, Program, Result, Workspace, WorkspaceId, WorkspaceName,
};

pub struct Organizer {
    workspaces: Vec<Workspace>,
}

pub struct NewWorkspaceParameters {
    pub name: String,
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
}

impl Organizer {
    pub fn add_command(&mut self, id: &WorkspaceId, command: NewCommandParameters) -> Result<()> {
        let NewCommandParameters { name, program } = command;
        let workspace = self.get_workspace_mut(id)?;
        let count = workspace.commands.len();

        workspace.commands.push(Command {
            id: CommandId::new(count),
            name: CommandName::new(name),
            program: Program::new(program),
        });

        Ok(())
    }

    pub fn add_workspace(&mut self, parameters: NewWorkspaceParameters) -> &Workspace {
        let NewWorkspaceParameters { name } = parameters;
        let count = self.workspaces.len();

        self.workspaces.push(Workspace {
            id: WorkspaceId::new(count),
            name: WorkspaceName::new(name),
            ..Default::default()
        });

        &self.workspaces[count]
    }

    pub fn delete_command(&mut self, w_id: &WorkspaceId, c_id: &CommandId) -> Result<()> {
        self.get_command(w_id, c_id)?;

        let workspace = self.get_workspace_mut(w_id)?;
        workspace.commands.remove(c_id.raw());
        workspace.update_command_ids();

        Ok(())
    }

    pub fn delete_workspace(&mut self, id: &WorkspaceId) -> Result<()> {
        self.get_workspace_mut(id)?;

        self.workspaces.remove(id.raw());
        self.update_workspace_ids();

        Ok(())
    }

    pub fn get_command(&self, w_id: &WorkspaceId, c_id: &CommandId) -> Result<&Command> {
        self.get_workspace(w_id)?
            .commands
            .get(c_id.raw())
            .ok_or(Error::NotFound("command".into()))
    }

    pub fn get_workspace(&self, id: &WorkspaceId) -> Result<&Workspace> {
        self.workspaces
            .get(id.raw())
            .ok_or(Error::NotFound("workspace".into()))
    }

    fn get_workspace_mut(&mut self, id: &WorkspaceId) -> Result<&mut Workspace> {
        self.workspaces
            .get_mut(id.raw())
            .ok_or(Error::NotFound("workspace".into()))
    }

    pub fn initialize() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }

    pub fn promote_workspace(&mut self, id: &WorkspaceId) -> Result<()> {
        self.get_workspace(id)?;

        let workspace = self.workspaces.remove(id.raw());
        self.workspaces.insert(0, workspace);
        self.update_workspace_ids();

        Ok(())
    }

    fn update_workspace_ids(&mut self) {
        self.workspaces
            .iter_mut()
            .enumerate()
            .for_each(|(index, workspace)| {
                workspace.id = WorkspaceId::new(index);
            });
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}
