use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub program: String,
}

mod converter {
    use crate::{data::Command as CommandData, data::Workspace as WorkspaceData};
    use hermione_memory::{
        Command, CommandName, CommandParameters, Program, Workspace, WorkspaceName,
        WorkspaceParameters,
    };

    impl From<&Workspace> for WorkspaceData {
        fn from(value: &Workspace) -> Self {
            WorkspaceData {
                name: value.name().to_string(),
                commands: value.commands().iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&Command> for CommandData {
        fn from(value: &Command) -> Self {
            CommandData {
                name: value.name().to_string(),
                program: value.program().to_string(),
            }
        }
    }

    impl From<WorkspaceData> for Workspace {
        fn from(value: WorkspaceData) -> Self {
            Workspace::new(WorkspaceParameters {
                name: WorkspaceName::new(value.name),
                commands: value.commands.into_iter().map(Into::into).collect(),
            })
        }
    }

    impl From<CommandData> for Command {
        fn from(value: CommandData) -> Self {
            Command::new(CommandParameters {
                program: Program::new(value.program),
                name: CommandName::new(value.name),
            })
        }
    }
}
