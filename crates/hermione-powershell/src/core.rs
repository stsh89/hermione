const DEFAULT_WINDOWS_TERMINAL_COMMAND: &str = "wt pwsh";

type Result<T> = std::result::Result<T, Error>;

pub trait Execute {
    fn execute(&self, input: &str) -> Result<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] eyre::Error),
}

pub struct PowerShellArguments {
    /// Executes the specified commands (and any parameters) as though they
    /// were typed at the PowerShell command prompt, and then exits, unless the
    /// NoExit parameter is specified.
    pub command: Option<String>,

    /// Does not exit after running startup commands.
    pub no_exit: bool,

    /// Sets the initial working directory by executing at startup.
    pub working_directory: Option<String>,
}

pub struct CopyToClipboard<'a, T>
where
    T: Execute,
{
    pub powershell: &'a T,
}

pub struct OpenWindowsTerminal<'a, T>
where
    T: Execute,
{
    pub powershell: &'a T,
}

impl<'a, T> CopyToClipboard<'a, T>
where
    T: Execute,
{
    pub fn execute(&self, text: &str) -> Result<()> {
        let input = format!("Set-Clipboard '{}'", text);

        self.powershell.execute(&input)
    }
}

impl<'a, T> OpenWindowsTerminal<'a, T>
where
    T: Execute,
{
    pub fn execute(&self, arguments: Option<PowerShellArguments>) -> Result<()> {
        let input = WindowsTerminalCommand { arguments }.into_powershell_input();

        self.powershell.execute(&input)
    }
}

struct WindowsTerminalCommand {
    arguments: Option<PowerShellArguments>,
}

impl WindowsTerminalCommand {
    fn into_powershell_input(self) -> String {
        let input = DEFAULT_WINDOWS_TERMINAL_COMMAND.to_string();

        let Some(arguments) = self.arguments else {
            return input;
        };

        let mut input = vec![input];

        let PowerShellArguments {
            command,
            no_exit,
            working_directory,
        } = arguments;

        if let Some(command) = command {
            input.push(format!("-Command {{{command}}}"));
        }

        if no_exit {
            input.push("-NoExit".into());
        }

        if let Some(working_directory) = working_directory {
            input.push(format!("-WorkingDirectory {working_directory}"));
        }

        input.join(" ")
    }
}
