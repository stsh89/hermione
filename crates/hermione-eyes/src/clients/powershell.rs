use crate::Result;
use hermione_wand::powershell;

pub struct Client {
    inner: powershell::Client,
}

pub struct StartWindowsTerminalParameters<'a> {
    pub command: Option<&'a str>,
    pub directory: Option<&'a str>,
    pub no_exit: bool,
}

impl Client {
    pub fn copy_to_clipboard(self, text: &str) -> Result<()> {
        self.inner.copy_to_clipboard(text)
    }

    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: powershell::Client::new()?,
        })
    }

    pub fn start_windows_terminal(self, parameters: StartWindowsTerminalParameters) -> Result<()> {
        self.inner.start_windows_terminal(parameters.into())
    }
}

impl<'a> From<StartWindowsTerminalParameters<'a>>
    for powershell::StartWindowsTerminalParameters<'a>
{
    fn from(parameters: StartWindowsTerminalParameters<'a>) -> Self {
        let StartWindowsTerminalParameters {
            command,
            directory,
            no_exit,
        } = parameters;

        Self {
            command,
            directory,
            no_exit,
        }
    }
}
