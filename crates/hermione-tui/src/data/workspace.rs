use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: usize,
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: usize,
    pub name: String,
    pub program: String,
}

mod converter {
    use crate::{data::Command as CommandData, data::Workspace as WorkspaceData};
    use hermione_memory::{Command, Workspace};

    impl From<&Workspace> for WorkspaceData {
        fn from(value: &Workspace) -> Self {
            WorkspaceData {
                id: value.id().raw(),
                name: value.name().to_string(),
                commands: value.commands().iter().map(Into::into).collect(),
            }
        }
    }

    impl From<&Command> for CommandData {
        fn from(value: &Command) -> Self {
            CommandData {
                id: value.id().raw(),
                name: value.name().to_string(),
                program: value.program().to_string(),
            }
        }
    }
}
