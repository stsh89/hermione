use crate::{
    definitions::{Command, CommandId, WorkspaceId},
    services::StorageProvider,
    Result,
};

pub trait CreateCommand: StorageProvider {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command>;
}

pub trait FindCommand: StorageProvider {
    fn find_command(&self, id: &CommandId) -> Result<Option<Command>>;
}

pub trait UpdateCommand: StorageProvider {
    fn update_command(&self, parameters: EditCommandParameters) -> Result<Command>;
}

pub struct EditCommandParameters<'a> {
    pub id: &'a CommandId,
    pub name: &'a str,
    pub program: &'a str,
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}
