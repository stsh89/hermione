pub use crate::Result;

pub trait ClipboardService {}

pub trait CopyCommandToClipboard: ClipboardService {
    fn copy_command_to_clipboard(&self, text: &str) -> Result<()>;
}
