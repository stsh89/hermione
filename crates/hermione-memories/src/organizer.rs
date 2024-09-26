use crate::{
    workspace::Location, Command, CommandName, Error, Number, Program, Result, Workspace,
    WorkspaceName,
};

pub struct Organizer {
    workspaces: Vec<Workspace>,
}

pub struct NewWorkspaceParameters {
    pub name: String,
}

pub struct CommandParameters {
    pub name: String,
    pub program: String,
}

impl Organizer {
    pub fn add_command(&mut self, number: Number, command: CommandParameters) -> Result<()> {
        let CommandParameters { name, program } = command;
        let workspace = self.get_workspace_mut(number)?;
        let count = workspace.commands.len();

        workspace.commands.push(Command {
            number: count.into(),
            name: CommandName::new(name),
            program: Program::new(program),
        });

        Ok(())
    }

    pub fn add_workspace(&mut self, parameters: NewWorkspaceParameters) -> &Workspace {
        let NewWorkspaceParameters { name } = parameters;
        let count = self.workspaces.len();

        self.workspaces.push(Workspace {
            number: count.into(),
            name: WorkspaceName::new(name),
            commands: vec![],
            location: None,
        });

        &self.workspaces[count]
    }

    pub fn delete_command(&mut self, w_number: Number, c_number: Number) -> Result<()> {
        self.get_command(w_number, c_number)?;

        let workspace = self.get_workspace_mut(w_number)?;
        workspace.commands.remove(c_number.into());
        workspace.update_command_numbers();

        Ok(())
    }

    pub fn delete_workspace(&mut self, number: Number) -> Result<()> {
        self.get_workspace_mut(number)?;

        self.workspaces.remove(number.into());
        self.update_workspace_numbers();

        Ok(())
    }

    pub fn get_command(&self, w_number: Number, c_number: Number) -> Result<&Command> {
        self.get_workspace(w_number)?
            .commands
            .get::<usize>(c_number.into())
            .ok_or(Error::NotFound("command".into()))
    }

    fn get_coomand_mut(&mut self, w_number: Number, c_number: Number) -> Result<&mut Command> {
        let workspace = self.get_workspace_mut(w_number)?;

        workspace
            .commands
            .get_mut::<usize>(c_number.into())
            .ok_or(Error::NotFound("workspace".into()))
    }

    pub fn get_workspace(&self, number: Number) -> Result<&Workspace> {
        self.workspaces
            .get::<usize>(number.into())
            .ok_or(Error::NotFound("workspace".into()))
    }

    fn get_workspace_mut(&mut self, number: Number) -> Result<&mut Workspace> {
        self.workspaces
            .get_mut::<usize>(number.into())
            .ok_or(Error::NotFound("workspace".into()))
    }

    pub fn initialize() -> Self {
        Self {
            workspaces: Vec::new(),
        }
    }

    pub fn rename_workspace(&mut self, number: Number, name: WorkspaceName) -> Result<()> {
        let workspace = self.get_workspace_mut(number)?;
        workspace.name = name;

        Ok(())
    }

    pub fn set_workspace_location(&mut self, number: Number, location: Location) -> Result<()> {
        let workspace = self.get_workspace_mut(number)?;
        workspace.location = Some(location);

        Ok(())
    }

    pub fn promote_command(&mut self, w_number: Number, c_number: Number) -> Result<()> {
        self.get_command(w_number, c_number)?;

        let workspace = self.get_workspace_mut(w_number)?;
        let command = workspace.commands.remove(c_number.into());

        workspace.commands.insert(0, command);
        workspace.update_command_numbers();

        Ok(())
    }

    pub fn promote_workspace(&mut self, number: Number) -> Result<()> {
        self.get_workspace(number)?;

        let workspace = self.workspaces.remove(number.into());

        self.workspaces.insert(0, workspace);
        self.update_workspace_numbers();

        Ok(())
    }

    fn update_workspace_numbers(&mut self) {
        self.workspaces
            .iter_mut()
            .enumerate()
            .for_each(|(index, workspace)| {
                workspace.number = index.into();
            });
    }

    pub fn update_command(
        &mut self,
        w_n: Number,
        c_n: Number,
        command: CommandParameters,
    ) -> Result<()> {
        let CommandParameters { name, program } = command;
        let command = self.get_coomand_mut(w_n, c_n)?;

        command.name = CommandName::new(name);
        command.program = Program::new(program);

        Ok(())
    }

    pub fn workspaces(&self) -> &[Workspace] {
        &self.workspaces
    }

    pub fn workspace_names(&self) -> Vec<&WorkspaceName> {
        self.workspaces
            .iter()
            .map(|workspace| workspace.name())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{NewWorkspaceParameters, Organizer, Result};

    fn empty_organizer() -> Organizer {
        Organizer::initialize()
    }

    fn filled_organizer() -> Organizer {
        let mut organizer = empty_organizer();

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Wksp 0".into(),
        });

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Wksp 1".into(),
        });

        organizer.add_workspace(NewWorkspaceParameters {
            name: "Wksp 2".into(),
        });

        organizer
    }

    #[test]
    fn test_add_workspace() {
        let mut organizer = empty_organizer();

        let workspace = organizer.add_workspace(NewWorkspaceParameters {
            name: "Wksp".into(),
        });

        assert_eq!(workspace.number(), 0.into());
        assert_eq!(workspace.name(), "Wksp");
    }

    #[test]
    fn test_promote_workspace() -> Result<()> {
        let mut organizer = filled_organizer();
        let workspace = organizer.get_workspace(1.into())?;

        assert_eq!(workspace.name(), "Wksp 1");
        assert_eq!(workspace.number(), 1.into());

        organizer.promote_workspace(1.into())?;

        assert_eq!(organizer.workspace_names(), ["Wksp 1", "Wksp 0", "Wksp 2"]);

        let workspace = organizer.get_workspace(0.into())?;

        assert_eq!(workspace.name(), "Wksp 1");

        Ok(())
    }
}
