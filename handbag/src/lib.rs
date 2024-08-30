mod command;
mod id;
mod operation;
mod organizer;
mod organizer_error;
mod workspace;

pub use command::{Command, Name as CommandName, Program};
pub use id::Id;
pub use operation::{Load, LoadOrganizer, Save, SaveOrganizer};
pub use organizer::Organizer;
pub use organizer_error::OrganizerError;
pub use workspace::{Name as WorkspaceName, Workspace};

pub type OrganizerResult<T> = Result<T, OrganizerError>;
