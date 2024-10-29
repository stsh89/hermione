use crate::{workspaces::WorkspaceId, Error, Result};
use chrono::{DateTime, Utc};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

pub trait CreateCommand {
    fn create_command(&self, command: Command) -> Result<Command>;
}

pub trait DeleteCommandFromWorkspace {
    fn delete(&self, workspace_id: &WorkspaceId, id: &CommandId) -> Result<()>;
}

pub trait GetCommandFromWorkspace {
    fn get_command_from_workspace(
        &self,
        workspace_id: &WorkspaceId,
        id: &CommandId,
    ) -> Result<Command>;
}

pub trait ListCommandsWithinWorkspace {
    fn list_commands_within_workspace(
        &self,
        parameters: ListCommandsWithinWorkspaceParameters,
    ) -> Result<Vec<Command>>;
}

pub trait UpdateCommand {
    fn update_command(&self, command: Command) -> Result<Command>;
}

pub struct CreateCommandOperation<'a, S> {
    pub creator: &'a S,
}

pub struct DeleteCommandFromWorkspaceOperation<'a, D> {
    pub deleter: &'a D,
}

pub struct GetCommandFromWorkspaceOperation<'a, R> {
    pub getter: &'a R,
}

pub struct ListCommandsWithinWorkspaceOperation<'a, L>
where
    L: ListCommandsWithinWorkspace,
{
    pub lister: &'a L,
}

pub struct UpdateCommandOperation<'a, GCP, UCP> {
    pub get_command_provider: &'a GCP,
    pub update_command_provider: &'a UCP,
}

#[derive(Debug)]
pub struct Command {
    last_execute_time: Option<DateTime<Utc>>,
    id: CommandId,
    name: CommandName,
    program: CommandProgram,
    workspace_id: WorkspaceId,
}

#[derive(Debug)]
struct CommandName {
    value: String,
}

#[derive(Debug)]
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

pub struct ListCommandsWithinWorkspaceParameters<'a> {
    pub page_number: u32,
    pub page_size: u32,
    pub program_contains: &'a str,
    pub workspace_id: Uuid,
}

pub struct UpdateCommandParameters<'a> {
    pub id: &'a CommandId,
    pub name: String,
    pub program: String,
    pub workspace_id: &'a WorkspaceId,
}

#[derive(Debug, PartialEq)]
pub struct CommandId(Uuid);

impl<'a, S> CreateCommandOperation<'a, S>
where
    S: CreateCommand,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        tracing::info!(operation = "Create command");

        if !command.id().is_nil() {
            return Err(Error::FailedPrecondition(
                "Command id is already set".to_string(),
            ));
        }

        let command = self.creator.create_command(command)?;

        if command.id().is_nil() {
            return Err(Error::Internal(
                "Failed to create command: id is not set".to_string(),
            ));
        };

        Ok(command)
    }
}

impl<'a, D> DeleteCommandFromWorkspaceOperation<'a, D>
where
    D: DeleteCommandFromWorkspace,
{
    pub fn execute(&self, workspace_id: &WorkspaceId, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Delete command from workspace");

        self.deleter.delete(workspace_id, id)
    }
}

impl<'a, R> GetCommandFromWorkspaceOperation<'a, R>
where
    R: GetCommandFromWorkspace,
{
    pub fn execute(&self, workspace_id: &WorkspaceId, id: &CommandId) -> Result<Command> {
        tracing::info!(operation = "Get command from workspace");

        self.getter.get_command_from_workspace(workspace_id, id)
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
        tracing::info!(operation = "List commands within workspace");

        self.lister.list_commands_within_workspace(parameters)
    }
}

impl<'a, GCP, UCP> UpdateCommandOperation<'a, GCP, UCP>
where
    GCP: GetCommandFromWorkspace,
    UCP: UpdateCommand,
{
    pub fn execute(&self, parameters: UpdateCommandParameters) -> Result<Command> {
        tracing::info!(operation = "Update command");

        let UpdateCommandParameters {
            id,
            name,
            program,
            workspace_id,
        } = parameters;

        let mut command = GetCommandFromWorkspaceOperation {
            getter: self.get_command_provider,
        }
        .execute(workspace_id, id)?;

        command.rename(name);
        command.change_program(program);

        self.update_command_provider.update_command(command)
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
            id: CommandId(id),
            name: CommandName { value: name },
            program: CommandProgram { value: program },
            workspace_id: workspace_id.into(),
        }
    }

    pub fn id(&self) -> &CommandId {
        &self.id
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
            id: CommandId(Uuid::nil()),
            name: CommandName { value: name },
            program: CommandProgram { value: program },
            workspace_id: workspace_id.into(),
        }
    }

    pub fn program(&self) -> &str {
        &self.program.value
    }

    pub fn rename(&mut self, name: String) {
        self.name = CommandName { value: name };
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<()> {
        if !self.id.is_nil() {
            return Err(Error::Internal("Command id is already set".to_string()));
        }

        self.id = CommandId(id);

        Ok(())
    }

    pub fn workspace_id(&self) -> &WorkspaceId {
        &self.workspace_id
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

impl Deref for CommandId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CommandId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let value: Uuid = s.parse().map_err(eyre::Error::new)?;

        Ok(Self(value))
    }
}

impl From<Uuid> for CommandId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
