use crate::{
    commands::{Command, CommandId, GetCommandFromWorkspace, GetCommandFromWorkspaceOperation},
    workspaces::{GetWorkspace, GetWorkspaceOperation, Workspace, WorkspaceId},
    Error, Result,
};

pub trait CopyCommandToClipboard {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()>;
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self, parameters: OpenWindowsTerminalParameters) -> Result<()>;
}

pub trait RunProgram {
    fn run(&self, parameters: RunProgramParameters) -> Result<()>;
}

pub trait TrackCommandExecutionTime {
    fn track_command_execution_time(&self, command: Command) -> Result<Command>;
}

pub trait TrackWorkspaceAccessTime {
    fn track_workspace_access_time(&self, workspace: Workspace) -> Result<Workspace>;
}

pub struct ExecuteCommandWithinWorkspaceParameters<'a> {
    pub id: &'a CommandId,
    pub workspace_id: &'a WorkspaceId,
    pub no_exit: bool,
}

pub struct RunProgramParameters<'a> {
    pub program: &'a str,
    pub no_exit: bool,
    pub working_directory: &'a str,
}

pub struct CopyCommandToClipboardOperation<'a, T, G> {
    pub getter: &'a G,
    pub clipboard_provider: &'a T,
}

pub struct ExecuteCommandOperation<'a, R, T, C, W, WT> {
    pub runner: &'a R,
    pub command_tracker: &'a T,
    pub workspace_tracker: &'a WT,
    pub get_command: &'a C,
    pub get_workspace: &'a W,
}

pub struct OpenWindowsTerminalParameters<'a> {
    pub working_directory: &'a str,
}

pub struct OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub windows_terminal_provider: &'a T,
}

pub struct TrackCommandExecutionTimeOperation<'a, T> {
    pub tracker: &'a T,
}

pub struct TrackWorkspaceAccessTimeOperation<'a, T> {
    pub tracker: &'a T,
}

impl<'a, T, G> CopyCommandToClipboardOperation<'a, T, G>
where
    T: CopyCommandToClipboard,
    G: GetCommandFromWorkspace,
{
    pub fn execute(&self, workspace_id: &WorkspaceId, id: &CommandId) -> Result<()> {
        tracing::info!(operation = "Copy command to clipboard");

        let command = self.getter.get_command_from_workspace(workspace_id, id)?;

        self.clipboard_provider
            .copy_command_to_clipboard(command.program())?;

        Ok(())
    }
}

impl<'a, R, T, G, W, WT> ExecuteCommandOperation<'a, R, T, G, W, WT>
where
    R: RunProgram,
    T: TrackCommandExecutionTime,
    G: GetCommandFromWorkspace,
    W: GetWorkspace,
    WT: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, parameters: ExecuteCommandWithinWorkspaceParameters) -> Result<Command> {
        tracing::info!(operation = "Execute command");

        let ExecuteCommandWithinWorkspaceParameters {
            id,
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
        .execute(workspace_id, id)?;

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

impl<'a, T> OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub fn execute(&self, parameters: OpenWindowsTerminalParameters) -> Result<()> {
        tracing::info!(operation = "Open Windows Terminal");

        self.windows_terminal_provider
            .open_windows_terminal(parameters)?;

        Ok(())
    }
}

impl<'a, T> TrackCommandExecutionTimeOperation<'a, T>
where
    T: TrackCommandExecutionTime,
{
    pub fn execute(&self, command: Command) -> Result<Command> {
        tracing::info!(operation = "Track command execution time");

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

impl<'a, T> TrackWorkspaceAccessTimeOperation<'a, T>
where
    T: TrackWorkspaceAccessTime,
{
    pub fn execute(&self, workspace: Workspace) -> Result<Workspace> {
        tracing::info!(operation = "Track workspace access time");

        let time = workspace.last_access_time().cloned();

        let workspace = self.tracker.track_workspace_access_time(workspace)?;
        let error_message = "Failed to track workspace access time".to_string();

        if let Some(new_time) = workspace.last_access_time() {
            if let Some(time) = time {
                if time >= *new_time {
                    return Err(Error::Internal(error_message));
                }
            }
        } else {
            return Err(Error::Internal(error_message));
        }

        Ok(workspace)
    }
}
