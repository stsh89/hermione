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

    pub fn workspace_ids(&self) -> Vec<&WorkspaceId> {
        self.workspaces
            .iter()
            .map(|workspace| workspace.id())
            .collect()
    }

    pub fn workspace_names(&self) -> Vec<&WorkspaceName> {
        self.workspaces
            .iter()
            .map(|workspace| workspace.name())
            .collect()
    }
}

mod tests {
    use super::{Result, NewWorkspaceParameters, Organizer, WorkspaceId, WorkspaceName, Workspace};

    fn empty_organizer() -> Organizer {
        Organizer::initialize()
    }

    fn filled_organizer() -> Organizer {
        let mut organizer = empty_organizer();

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Test Workspace 0".into(),
        });

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Test Workspace 1".into(),
        });

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Test Workspace 2".into(),
        });

        organizer
    }

    fn workspace_names(workspaces: &[Workspace]) -> Vec<&WorkspaceName> {
        workspaces
            .iter()
            .map(|workspace| workspace.name())
            .collect()
    }

    #[test]
    fn test_add_workspace() {
        let mut organizer = empty_organizer();

        let workspace = organizer.add_workspace(NewWorkspaceParameters {
            name: "Test Workspace".into(),
        });

        assert_eq!(workspace.id(), &0);
        assert_eq!(workspace.name(), "Test Workspace");
    }

    #[test]
    fn test_promote_workspace() -> Result<()> {
        let mut organizer = filled_organizer();
        let workspace = organizer.get_workspace(&WorkspaceId::new(1))?;

        assert_eq!(workspace.name(), "Test Workspace 1");
        assert_eq!(workspace.id(), &1);

        organizer.promote_workspace(&WorkspaceId::new(1))?;

        assert_eq!(organizer.workspace_names(), ["Test Workspace 1", "Test Workspace 0", "Test Workspace 2"]);
        assert_eq!(organizer.workspace_ids(), [&0,&1,&2]);

        let workspace = organizer.get_workspace(&WorkspaceId::new(0))?;

        assert_eq!(workspace.name(), "Test Workspace 1");
        assert_eq!(workspace.id(), &0);

        Ok(())
    }
}
