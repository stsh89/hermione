mod command;
mod error;
mod number;
mod operation;
mod organizer;
mod workspace;

pub use command::{Command, Name as CommandName, Program};
pub use error::Error;
pub use number::Number;
pub use operation::{Load, LoadOrganizer, Save, SaveOrganizer};
pub use organizer::{CommandParameters, NewWorkspaceParameters, Organizer};
pub use workspace::{Name as WorkspaceName, Workspace};

pub type Result<T> = std::result::Result<T, Error>;
