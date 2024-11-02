pub use crate::Result;

pub trait ClipboardProvider {}

pub trait CopyCommandToClipboard: ClipboardProvider {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()>;
}
