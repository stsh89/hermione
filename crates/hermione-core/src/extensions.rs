use crate::Result;

pub struct OpenWindowsTerminalParameters<'a> {
    pub working_directory: &'a str,
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self, parameters: OpenWindowsTerminalParameters) -> Result<()>;
}

pub struct OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub windows_terminal_provider: &'a T,
}

impl<'a, T> OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub fn execute(&self, parameters: OpenWindowsTerminalParameters) -> Result<()> {
        self.windows_terminal_provider
            .open_windows_terminal(parameters)?;

        Ok(())
    }
}
