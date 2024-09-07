mod command;
mod error;
mod id;
mod operation;
mod organizer;
mod workspace;

pub use command::{Command, Name as CommandName, Program};
pub use error::Error;
pub use id::Id;
pub use operation::{Load, LoadOrganizer, Save, SaveOrganizer};
pub use organizer::Organizer;
pub use workspace::{Name as WorkspaceName, Workspace};

pub type Result<T> = std::result::Result<T, Error>;
