use eyre::{eyre, Report};
use hermione_nexus::{Error, Result};
use std::{
    io,
    process::{Child, Command, Stdio},
    sync::RwLock,
};

const DEFAULT_WINDOWS_TERMINAL_COMMAND_TEXT: &str = "wt pwsh";
const POWERSHELL_COMMAND_TEXT: &str = "pwsh";

pub struct PowerShellProcess {
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

impl PowerShellProcess {
    pub fn spawn() -> io::Result<Self> {
        let mut cmd = Command::new(POWERSHELL_COMMAND_TEXT);

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;

        Ok(Self {
            child: RwLock::new(child),
        })
    }
}

pub fn copy_to_clipboard(conn: &PowerShellProcess, text: &str) -> Result<()> {
    let text = copy_to_clipboard_command_text(text);

    execute(conn, &text)
}

pub fn open_windows_terminal(
    conn: &PowerShellProcess,
    parameters: Option<PowerShellParameters>,
) -> Result<()> {
    let text = open_windows_termina_command_text(parameters);

    execute(conn, &text)
}

fn copy_to_clipboard_command_text(text: &str) -> String {
    format!("Set-Clipboard '{}'", text)
}

fn open_windows_termina_command_text(parameters: Option<PowerShellParameters>) -> String {
    let cmd = DEFAULT_WINDOWS_TERMINAL_COMMAND_TEXT.to_string();

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

fn execute(conn: &PowerShellProcess, program: &str) -> Result<()> {
    let mut child = conn
        .child
        .write()
        .map_err(|err| Report::msg(err.to_string()).wrap_err("Could not access PowerShell process"))
        .map_err(Error::unexpected)?;

    let stdin = child
        .stdin
        .as_mut()
        .ok_or(eyre!(
            "Could not access standard input stream of the PowerShell process",
        ))
        .map_err(Error::unexpected)?;

    use io::Write;
    writeln!(stdin, "{}", &program)
        .map_err(|err| {
            Report::new(err)
                .wrap_err("Could not write into standard input stream of the PowerShell process")
        })
        .map_err(Error::unexpected)?;

    Ok(())
}
