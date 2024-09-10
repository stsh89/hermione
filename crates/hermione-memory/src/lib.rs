mod command;
mod error;
mod operation;
mod organizer;
mod workspace;

pub use command::{Command, Id as CommandId, Name as CommandName, Program};
pub use error::Error;
pub use operation::{Load, LoadOrganizer, Save, SaveOrganizer};
pub use organizer::{NewCommandParameters, NewWorkspaceParameters, Organizer};
pub use workspace::{Id as WorkspaceId, Name as WorkspaceName, Workspace};

pub type Result<T> = std::result::Result<T, Error>;
