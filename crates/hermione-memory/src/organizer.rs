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

    pub fn add_workspace(&mut self, parameters: NewWorkspaceParameters) -> WorkspaceId {
        let NewWorkspaceParameters { name } = parameters;
        let count = self.workspaces.len();

        self.workspaces.push(Workspace {
            id: WorkspaceId::new(count),
            name: WorkspaceName::new(name),
            ..Default::default()
        });

        WorkspaceId::new(count)
    }

    pub fn delete_command(&mut self, w_id: &WorkspaceId, c_id: &CommandId) -> Result<()> {
        let workspace = self.get_workspace_mut(w_id)?;

        if let Some(_command) = workspace.commands.get(c_id.raw()) {
            workspace.commands.remove(c_id.raw());
            workspace
                .commands
                .iter_mut()
                .enumerate()
                .for_each(|(index, command)| {
                    command.id = CommandId::new(index);
                })
        } else {
            return Err(Error::NotFound("command".into()));
        }

        Ok(())
    }

    pub fn delete_workspace(&mut self, id: &WorkspaceId) -> Result<()> {
        self.get_workspace_mut(id)?;
        self.workspaces.remove(id.raw());

        self.workspaces
            .iter_mut()
            .enumerate()
            .for_each(|(index, workspace)| {
                workspace.id = WorkspaceId::new(index);
            });

        Ok(())
    }

    pub fn initialize() -> Self {
        Self {
            workspaces: Vec::new(),
        }
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

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }
}
