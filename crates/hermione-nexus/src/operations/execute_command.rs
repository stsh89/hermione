use crate::{
    definitions::{Command, CommandId},
    operations::GetCommandOperation,
    services::{
        FindCommand, RunProgram, StorageProvider, SystemProvider, TrackCommandExecuteTime,
        TrackWorkspaceAccessTime,
    },
    Result,
};

pub struct ExecuteCommandOperation<'a, FCP, SP, TCP, TWP>
where
    FCP: StorageProvider,
    SP: SystemProvider,
    TCP: StorageProvider,
    TWP: StorageProvider,
{
    pub find_command_provider: &'a FCP,
    pub system_provider: &'a SP,
    pub track_command_provider: &'a TCP,
    pub track_workspace_provider: &'a TWP,
}

impl<'a, FCP, SP, TCP, TWP> ExecuteCommandOperation<'a, FCP, SP, TCP, TWP>
where
    FCP: FindCommand,
    SP: RunProgram,
    TCP: TrackCommandExecuteTime,
    TWP: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Execute command");

        let command = self.get_command(id)?;

        self.system_provider.run_program(command.program())?;
        self.track_command_provider.track_command_execute_time(id)?;
        self.track_workspace_provider
            .track_workspace_access_time(command.workspace_id())?;

        Ok(())
    }

    fn get_command(&self, id: &CommandId) -> Result<Command> {
        GetCommandOperation {
            provider: self.find_command_provider,
        }
        .execute(id)
    }
}
