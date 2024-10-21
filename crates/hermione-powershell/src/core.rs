type Result<T> = std::result::Result<T, Error>;

pub trait CopyToClipboard {
    fn copy_to_clipboard(&self, text: &str) -> Result<()>;
}

pub trait OpenWindowsTerminal {
    fn open_windows_terminal(&self) -> Result<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] eyre::Error),
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
    pub fn execute(&self, text: &str) -> Result<()> {
        self.powershell.copy_to_clipboard(text)
    }
}

impl<'a, T> OpenWindowsTerminalOperation<'a, T>
where
    T: OpenWindowsTerminal,
{
    pub fn execute(&self) -> Result<()> {
        self.powershell.open_windows_terminal()
    }
}
