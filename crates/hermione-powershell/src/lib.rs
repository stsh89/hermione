use std::{
    io,
    process::{Child, Command, Stdio},
    sync::RwLock,
};

const DEFAULT_WINDOWS_TERMINAL_COMMAND: &str = "wt pwsh";

pub trait CopyToClipboard {
    fn copy_to_clipboard(&self, text: &str) -> eyre::Result<()>;
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self) -> eyre::Result<()>;
}

pub struct CopyToClipboardOperation<'a, T>
where
    T: CopyToClipboard,
{
    pub powershell: &'a T,
}

pub struct OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub powershell: &'a T,
}

impl<'a, T> CopyToClipboardOperation<'a, T>
where
    T: CopyToClipboard,
{
    pub fn execute(&self, text: &str) -> eyre::Result<()> {
        self.powershell.copy_to_clipboard(text)
    }
}

impl<'a, T> OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub fn execute(&self) -> eyre::Result<()> {
        self.powershell.open_windows_terminal()
    }
}

pub struct Client {
    child: RwLock<Child>,
}

pub struct PowerShellParameters<'a> {
    /// Executes the specified commands (and any parameters) as though they
    /// were typed at the PowerShell command prompt, and then exits, unless the
    /// NoExit parameter is specified.
    pub command: Option<&'a str>,

    /// Does not exit after running startup commands.
    pub no_exit: bool,

    /// Sets the initial working directory by executing at startup.
    pub working_directory: Option<&'a str>,
}

impl Client {
    fn clipboard_provider(&self) -> ClipboardProvider {
        ClipboardProvider { child: &self.child }
    }

    pub fn copy_to_clipboard(&self, text: &str) -> anyhow::Result<()> {
        CopyToClipboardOperation {
            powershell: &self.clipboard_provider(),
        }
        .execute(text)
        .map_err(|err| anyhow::anyhow!(err))?;

        Ok(())
    }

    pub fn new() -> anyhow::Result<Self> {
        let mut cmd = Command::new("pwsh");

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;

        Ok(Self {
            child: RwLock::new(child),
        })
    }

    pub fn open_windows_terminal(
        &self,
        parameters: Option<PowerShellParameters>,
    ) -> anyhow::Result<()> {
        OpenWindowsTerminalOperation {
            powershell: &self.windows_terminal_provider(parameters),
        }
        .execute()
        .map_err(|err| anyhow::anyhow!(err))?;

        Ok(())
    }

    fn windows_terminal_provider(
        &self,
        parameters: Option<PowerShellParameters>,
    ) -> WindowsTerminalProvider {
        WindowsTerminalProvider::new(&self.child, parameters)
    }
}

struct ClipboardProvider<'a> {
    child: &'a RwLock<Child>,
}

struct WindowsTerminalProvider<'a> {
    child: &'a RwLock<Child>,
    command: String,
}

impl<'a> ClipboardProvider<'a> {
    fn copy(&self, text: &str) -> eyre::Result<()> {
        let command = format!("Set-Clipboard '{}'", text);

        execute(self.child, &command)
    }
}

impl<'a> WindowsTerminalProvider<'a> {
    fn new(child: &'a RwLock<Child>, parameters: Option<PowerShellParameters>) -> Self {
        Self {
            child,
            command: open_windows_terminal_command(parameters),
        }
    }

    fn open_windows_terminal(&self) -> eyre::Result<()> {
        execute(self.child, &self.command)
    }
}

impl<'a> CopyToClipboard for ClipboardProvider<'a> {
    fn copy_to_clipboard(&self, text: &str) -> eyre::Result<()> {
        self.copy(text)
    }
}

impl<'a> OpenWindowsTerminal for WindowsTerminalProvider<'a> {
    fn open_windows_terminal(&self) -> eyre::Result<()> {
        self.open_windows_terminal()
    }
}

fn execute(child: &RwLock<Child>, command: &str) -> eyre::Result<()> {
    let mut child = child.write().map_err(|err| eyre::eyre!("{}", err))?;

    let stdin = child.stdin.as_mut().ok_or(eyre::eyre!(
        "Can't obtain handle for writing to the PowerShell's standard input"
    ))?;

    use io::Write;
    writeln!(stdin, "{}", &command).map_err(eyre::Error::new)?;

    Ok(())
}

fn open_windows_terminal_command(parameters: Option<PowerShellParameters>) -> String {
    let cmd = DEFAULT_WINDOWS_TERMINAL_COMMAND.to_string();

    let Some(parameters) = parameters else {
        return cmd;
    };

    let mut cmd = vec![cmd];

    let PowerShellParameters {
        command,
        no_exit,
        working_directory,
    } = parameters;

    if let Some(command) = command {
        cmd.push(format!("-Command {{{command}}}"));
    }

    if no_exit {
        cmd.push("-NoExit".into());
    }

    if let Some(working_directory) = working_directory {
        cmd.push(format!("-WorkingDirectory {working_directory}"));
    }

    cmd.join(" ")
}
