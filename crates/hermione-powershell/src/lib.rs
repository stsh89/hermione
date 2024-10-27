use std::{
    io,
    process::{Child, Command, Stdio},
    sync::RwLock,
};

const DEFAULT_WINDOWS_TERMINAL_COMMAND: &str = "wt pwsh";
const POWERSHELL_COMMAND: &str = "pwsh";

pub struct PowerShellClient {
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

impl PowerShellClient {
    pub fn copy_to_clipboard(&self, text: &str) -> io::Result<()> {
        let command = format!("Set-Clipboard '{}'", text);
        self.execute(&command)
    }

    fn execute(&self, command: &str) -> io::Result<()> {
        let mut child = self
            .child
            .write()
            .map_err(|_err| io::Error::other("Failed to obtain PowerShell child processlock"))?;

        execute(&mut child, command)
    }

    pub fn new() -> io::Result<Self> {
        let child = spawn()?;

        Ok(Self {
            child: RwLock::new(child),
        })
    }

    pub fn open_windows_terminal(
        &self,
        parameters: Option<PowerShellParameters>,
    ) -> io::Result<()> {
        let command = open_windows_terminal_command(parameters);
        self.execute(&command)
    }
}

fn execute(child: &mut Child, command: &str) -> io::Result<()> {
    let stdin = child.stdin.as_mut().ok_or(io::Error::other(
        "Can't obtain handle for writing to the PowerShell's standard input",
    ))?;

    use io::Write;
    writeln!(stdin, "{}", &command)
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

fn spawn() -> io::Result<Child> {
    let mut cmd = Command::new(POWERSHELL_COMMAND);

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    cmd.spawn()
}
