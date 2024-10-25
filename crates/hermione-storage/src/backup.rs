use hermione_ops::{
    backup::{ImportCommand, ImportWorkspace},
    commands::Command,
    workspaces::Workspace,
    Error,
};

use crate::database::{CommandRecord, DatabaseProvider, WorkspaceRecord};

impl ImportCommand for DatabaseProvider {
    fn import_command(&self, command: Command) -> Result<Command, Error> {
        let record = CommandRecord::from_entity(&command)?;

        self.insert_command(record).map_err(eyre::Error::new)?;

        Ok(command)
    }
}

impl ImportWorkspace for DatabaseProvider {
    fn import_workspace(&self, entity: Workspace) -> Result<Workspace, Error> {
        let record = WorkspaceRecord::try_from(&entity)?;

        self.insert_workspace(record).map_err(eyre::Error::new)?;

        Ok(entity)
    }
}
