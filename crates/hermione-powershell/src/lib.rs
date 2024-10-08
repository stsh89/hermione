use std::{
    fmt, io,
    process::{Child, Command, Stdio},
};

type Result<T> = std::result::Result<T, io::Error>;

pub struct Client {
    powershell: Child,
}

pub struct WindowsTerminalParameters<'a> {
    pub command: Option<&'a str>,
    pub directory: Option<&'a str>,
    pub no_exit: bool,
}

struct WindowsTerminalCommand {
    command: String,
}

impl Default for WindowsTerminalCommand {
    fn default() -> Self {
        Self {
            command: "wt pwsh".to_string(),
        }
    }
}

impl fmt::Display for WindowsTerminalCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl WindowsTerminalCommand {
    fn with_working_directory(mut self, directory: Option<&str>) -> Self {
        if let Some(directory) = directory {
            let arg = format!(" -WorkingDirectory {directory}");

            self.command.push_str(&arg);
        }

        self
    }

    fn with_no_exit(mut self, no_exit: bool) -> Self {
        if no_exit {
            self.command.push_str(" -NoExit");
        }

        self
    }

    fn with_command(mut self, command: Option<&str>) -> Self {
        if let Some(command) = command {
            self.command.push_str(&format!(" -Command {{{command}}}"));
        }

        self
    }
}

impl Client {
    pub fn copy_to_clipboard(&mut self, text: &str) -> Result<()> {
        let command = format!("Set-Clipboard '{}'", text);

        self.write_stdin(&command)?;

        Ok(())
    }

    pub fn new() -> Result<Self> {
        let mut cmd = Command::new("pwsh");

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        Ok(Self {
            powershell: cmd.spawn()?,
        })
    }

    pub fn start_windows_terminal(&mut self, parameters: WindowsTerminalParameters) -> Result<()> {
        let WindowsTerminalParameters {
            directory,
            no_exit,
            command,
        } = parameters;

        let command = WindowsTerminalCommand::default()
            .with_working_directory(directory)
            .with_no_exit(no_exit)
            .with_command(command);

        self.write_stdin(&command.to_string())?;

        Ok(())
    }

    fn write_stdin(&mut self, input: &str) -> Result<()> {
        let stdin = self.powershell.stdin.as_mut().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Powershell stdin not found",
        ))?;

        use io::Write;
        writeln!(stdin, "{}", input)?;

        Ok(())
    }
}
