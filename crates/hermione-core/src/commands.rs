use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{Error, Result};

pub trait CreateCommand {
    fn create(&self, command: Command) -> Result<Command>;
}

pub trait DeleteCommandFromWorkspace {
    fn delete(&self, id: CommandWorkspaceScopeId) -> Result<()>;
}

pub trait FindCommandInWorkspace {
    fn find(&self, id: CommandWorkspaceScopeId) -> Result<Option<Command>>;
}

pub trait GetCommandInWorkspace {
    fn get(&self, id: CommandWorkspaceScopeId) -> Result<Command>;
}

pub trait ImportCommand {
    fn import(&self, command: Command) -> Result<Command>;
}

pub trait ListCommands {
    fn list(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>>;
}

pub trait ListCommandsWithinWorkspace {
    fn list(&self, parameters: ListCommandsWithinWorkspaceParameters) -> Result<Vec<Command>>;
}

pub trait TrackCommandExecutionTime {
    fn track_command_execution_time(&self, command: Command) -> Result<Command>;
}

pub trait UpdateCommand {
    fn update(&self, command: Command) -> Result<Command>;
}

pub struct CreateCommandOperation<'a, S> {
    pub creator: &'a S,
}

pub struct DeleteCommandFromWorkspaceOperation<'a, D> {
    pub deleter: &'a D,
}

pub struct FindCommandInWorkspaceOperation<'a, R> {
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

pub struct CommandWorkspaceScopeId {
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

impl<'a, D> DeleteCommandFromWorkspaceOperation<'a, D>
where
    D: DeleteCommandFromWorkspace,
{
    pub fn execute(&self, id: CommandWorkspaceScopeId) -> Result<()> {
        self.deleter.delete(id)
    }
}

impl<'a, R> FindCommandInWorkspaceOperation<'a, R>
where
    R: FindCommandInWorkspace,
{
    pub fn execute(&self, id: CommandWorkspaceScopeId) -> Result<Option<Command>> {
        self.finder.find(id)
    }
}

impl<'a, R> GetCommandFromWorkspaceOperation<'a, R>
where
    R: GetCommandInWorkspace,
{
    pub fn execute(&self, id: CommandWorkspaceScopeId) -> Result<Command> {
        self.getter.get(id)
    }
}

impl<'a, S> ImportCommandOperation<'a, S>
where
    S: ImportCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        self.importer.import(command)
    }
}

impl<'a, L> ListCommandsOperation<'a, L>
where
    L: ListCommands,
{
    pub fn execute(&self, parameters: ListCommandsParameters) -> Result<Vec<Command>> {
        self.lister.list(parameters)
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
        self.lister.list(parameters)
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
        self.updater.update(command)
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
