use hermione_coordinator::{commands, workspaces};
use hermione_notion_serde::de;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Command {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Workspace ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub workspace_id: String,

    #[serde(
        rename(deserialize = "Program"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub program: String,
}

#[derive(Deserialize)]
pub struct Workspace {
    #[serde(
        rename(deserialize = "Name"),
        deserialize_with = "de::title::deserializer"
    )]
    pub name: String,

    #[serde(
        rename(deserialize = "External ID"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub external_id: String,

    #[serde(
        rename(deserialize = "Location"),
        deserialize_with = "de::rich_text::deserializer"
    )]
    pub location: String,
}

impl PartialEq<workspaces::Dto> for Workspace {
    fn eq(&self, other: &workspaces::Dto) -> bool {
        self.name == other.name && self.location == other.location.as_deref().unwrap_or_default()
    }
}

impl PartialEq<workspaces::commands::Dto> for Command {
    fn eq(&self, other: &workspaces::commands::Dto) -> bool {
        self.name == other.name && self.program == other.program
    }
}

impl PartialEq<commands::Dto> for Command {
    fn eq(&self, other: &commands::Dto) -> bool {
        self.name == other.name && self.program == other.program
    }
}

impl From<Command> for commands::Dto {
    fn from(command: Command) -> Self {
        let Command {
            name,
            external_id,
            program,
            workspace_id,
        } = command;

        commands::Dto {
            id: external_id,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<Command> for workspaces::commands::Dto {
    fn from(command: Command) -> Self {
        let Command {
            name,
            external_id,
            program,
            workspace_id,
        } = command;

        workspaces::commands::Dto {
            id: external_id,
            last_execute_time: None,
            name,
            program,
            workspace_id,
        }
    }
}

impl From<Workspace> for workspaces::Dto {
    fn from(workspace: Workspace) -> Self {
        let Workspace {
            name,
            external_id,
            location,
        } = workspace;

        workspaces::Dto {
            id: external_id,
            last_access_time: None,
            location: Some(location),
            name,
        }
    }
}
