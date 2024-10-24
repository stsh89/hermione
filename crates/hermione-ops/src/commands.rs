use std::future::Future;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    workspaces::{
        GetWorkspace, GetWorkspaceOperation, TrackWorkspaceAccessTime,
        TrackWorkspaceAccessTimeOperation,
    },
    Error, Result,
};

pub trait CreateCommand {
    fn create(&self, command: Command) -> Result<Command>;
}

pub trait CopyToClipboard {
    fn copy_to_clipboard(&self, text: &str) -> Result<()>;
}

pub trait DeleteCommandFromWorkspace {
    fn delete(&self, id: CommandWorkspaceScopedId) -> Result<()>;
}

pub trait FindCommand {
    fn find_command(&self, id: Uuid) -> Result<Option<Command>>;
}

pub trait GetCommandFromWorkspace {
    fn get_command_from_workspace(&self, id: CommandWorkspaceScopedId) -> Result<Command>;
}

pub trait ImportCommand {
    fn import_command(&self, command: Command) -> Result<Command>;
}

pub trait ListCommands {
    fn list_commands(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListCommandsWithinWorkspace {
    fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceParameters,
    ) -> Result<Vec<Command>>;
}

pub trait ListAllCommandsInBatches {
    fn list_all_commands_in_batches(
        &self,
        batch_fn: impl Fn(Vec<Command>) -> Result<()>,
    ) -> impl Future<Output = Result<()>>;
}

pub trait TrackCommandExecutionTime {
    fn track_command_execution_time(&self, command: Command) -> Result<Command>;
}

pub trait RunProgram {
    fn run(&self, parameters: RunProgramParameters) -> Result<()>;
}

pub trait UpdateCommand {
    fn update_command(&self, command: Command) -> Result<Command>;
}

pub struct CreateCommandOperation<'a, S> {
    pub creator: &'a S,
}

pub struct CopyProgramToClipboardOperation<'a, T, G>
where
    T: CopyToClipboard,
{
    pub getter: &'a G,
    pub clipboard_provider: &'a T,
}

pub struct DeleteCommandFromWorkspaceOperation<'a, D> {
    pub deleter: &'a D,
}

pub struct ExecuteCommandWithinWorkspaceOperation<'a, R, T, C, W, WT> {
    pub runner: &'a R,
    pub command_tracker: &'a T,
    pub workspace_tracker: &'a WT,
    pub get_command: &'a C,
    pub get_workspace: &'a W,
}

pub struct FindCommandOperation<'a, R> {
    pub finder: &'a R,
}

pub struct GetCommandFromWorkspaceOperation<'a, R> {
    pub getter: &'a R,
}

pub struct ImportCommandOperation<'a, S> {
    pub importer: &'a S,
}

pub struct ListCommandsOperation<'a, L>
where
    L: ListCommands,
{
    pub lister: &'a L,
}

pub struct ListCommandsWithinWorkspaceOperation<'a, L>
where
    L: ListCommandsWithinWorkspace,
{
    pub lister: &'a L,
}

pub struct TrackCommandExecutionTimeOperation<'a, T> {
    pub tracker: &'a T,
}

pub struct UpdateCommandOperation<'a, U> {
    pub updater: &'a U,
}

pub struct Command {
    last_execute_time: Option<DateTime<Utc>>,
    id: Option<Uuid>,
    name: CommandName,
    program: CommandProgram,
    workspace_id: Uuid,
}

struct CommandName {
    value: String,
}

struct CommandProgram {
    value: String,
}

pub struct ExecuteCommandWithinWorkspaceParameters {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
    pub no_exit: bool,
}

pub struct LoadCommandParameters {
    pub last_execute_time: Option<DateTime<Utc>>,
    pub id: Uuid,
    pub name: String,
    pub program: String,
    pub workspace_id: Uuid,
}

pub struct NewCommandParameters {
    pub name: String,
    pub program: String,
    pub workspace_id: Uuid,
}

pub struct CommandWorkspaceScopedId {
    pub command_id: Uuid,
    pub workspace_id: Uuid,
}

pub struct ListCommandsParameters {
    pub page_number: u32,
    pub page_size: u32,
}

pub struct ListCommandsWithinWorkspaceParameters<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: Uuid,
}

pub struct RunProgramParameters<'a> {
    pub program: &'a str,
    pub no_exit: bool,
    pub working_directory: &'a str,
}

impl<'a, S> CreateCommandOperation<'a, S>
where
    S: CreateCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        if command.id().is_some() {
            return Err(Error::FailedPrecondition(
                "Command id is already set".to_string(),
            ));
        }

        let command = self.creator.create(command)?;

        if command.id().is_none() {
            return Err(Error::Internal(
                "Failed to create command: command id is not set".to_string(),
            ));
        };

        Ok(command)
    }
}

impl<'a, T, G> CopyProgramToClipboardOperation<'a, T, G>
where
    T: CopyToClipboard,
    G: GetCommandFromWorkspace,
{
    pub fn execute(&self, scoped_id: CommandWorkspaceScopedId) -> Result<()> {
        let command = self.getter.get_command_from_workspace(scoped_id)?;
        self.clipboard_provider
            .copy_to_clipboard(command.program())?;

        Ok(())
    }
}

impl<'a, D> DeleteCommandFromWorkspaceOperation<'a, D>
where
    D: DeleteCommandFromWorkspace,
{
    pub fn execute(&self, id: CommandWorkspaceScopedId) -> Result<()> {
        self.deleter.delete(id)
    }
}

impl<'a, R, T, G, W, WT> ExecuteCommandWithinWorkspaceOperation<'a, R, T, G, W, WT>
where
    R: RunProgram,
    T: TrackCommandExecutionTime,
    G: GetCommandFromWorkspace,
    W: GetWorkspace,
    WT: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, parameters: ExecuteCommandWithinWorkspaceParameters) -> Result<Command> {
        let ExecuteCommandWithinWorkspaceParameters {
            command_id,
            workspace_id,
            no_exit,
        } = parameters;

        let workspace = GetWorkspaceOperation {
            getter: self.get_workspace,
        }
        .execute(workspace_id)?;

        let command = GetCommandFromWorkspaceOperation {
            getter: self.get_command,
        }
        .execute(CommandWorkspaceScopedId {
            command_id,
            workspace_id,
        })?;

        self.runner.run(RunProgramParameters {
            program: command.program(),
            no_exit,
            working_directory: workspace.location().unwrap_or_default(),
        })?;

        let command = TrackCommandExecutionTimeOperation {
            tracker: self.command_tracker,
        }
        .execute(command)?;

        TrackWorkspaceAccessTimeOperation {
            tracker: self.workspace_tracker,
        }
        .execute(workspace)?;

        Ok(command)
    }
}

impl<'a, R> FindCommandOperation<'a, R>
where
    R: FindCommand,
{
    pub fn execute(&self, id: Uuid) -> Result<Option<Command>> {
        self.finder.find_command(id)
    }
}

impl<'a, R> GetCommandFromWorkspaceOperation<'a, R>
where
    R: GetCommandFromWorkspace,
{
    pub fn execute(&self, scoped_id: CommandWorkspaceScopedId) -> Result<Command> {
        self.getter.get_command_from_workspace(scoped_id)
    }
}

impl<'a, S> ImportCommandOperation<'a, S>
where
    S: ImportCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        self.importer.import_command(command)
    }
}

impl<'a, L> ListCommandsOperation<'a, L>
where
    L: ListCommands,
{
    pub fn execute(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>> {
        self.lister.list_commands(parameters)
    }
}

impl<'a, L> ListCommandsWithinWorkspaceOperation<'a, L>
where
    L: ListCommandsWithinWorkspace,
{
    pub fn execute(
        &self,
        parameters: ListCommandsWithinWorkspaceParameters,
    ) -> Result<Vec<Command>> {
        self.lister.list_commands_within_workspace(parameters)
    }
}

impl<'a, T> TrackCommandExecutionTimeOperation<'a, T>
where
    T: TrackCommandExecutionTime,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        let time = command.last_execute_time().cloned();

        let command = self.tracker.track_command_execution_time(command)?;
        let error_message = "Failed to track command execution time".to_string();

        if let Some(new_time) = command.last_execute_time() {
            if let Some(time) = time {
                if time >= *new_time {
                    return Err(Error::Internal(error_message));
                }
            }
        } else {
            return Err(Error::Internal(error_message));
        }

        Ok(command)
    }
}

impl<'a, U> UpdateCommandOperation<'a, U>
where
    U: UpdateCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        self.updater.update_command(command)
    }
}

impl Command {
    pub fn change_program(&mut self, program: String) {
        self.program = CommandProgram { value: program };
    }

    pub fn last_execute_time(&self) -> Option<&DateTime<Utc>> {
        self.last_execute_time.as_ref()
    }

    pub fn load(parameters: LoadCommandParameters) -> Self {
        let LoadCommandParameters {
            id,
            last_execute_time,
            name,
            program,
            workspace_id,
        } = parameters;

        Self {
            last_execute_time,
            id: Some(id),
            name: CommandName { value: name },
            program: CommandProgram { value: program },
            workspace_id,
        }
    }

    pub fn id(&self) -> Option<Uuid> {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name.value
    }

    pub fn new(parameters: NewCommandParameters) -> Self {
        let NewCommandParameters {
            name,
            program,
            workspace_id,
        } = parameters;

        Self {
            last_execute_time: None,
            id: None,
            name: CommandName { value: name },
            program: CommandProgram { value: program },
            workspace_id,
        }
    }

    pub fn program(&self) -> &str {
        &self.program.value
    }

    pub fn rename(&mut self, name: String) {
        self.name = CommandName { value: name };
    }

    pub fn try_id(&self) -> Result<Uuid> {
        self.id.ok_or(Error::DataLoss("Missing command ID".into()))
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::Internal("Command id is already set".to_string()));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn workspace_id(&self) -> Uuid {
        self.workspace_id
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name.value == other.name.value
            && self.program.value == other.program.value
            && self.workspace_id == other.workspace_id
    }
}
