use crate::{
    definitions::{Command, WorkspaceId},
    services::StorageProvider,
    Result,
};

pub trait CreateCommand: StorageProvider {
    fn create_command(&self, parameters: NewCommandParameters) -> Result<Command>;
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: WorkspaceId,
}
