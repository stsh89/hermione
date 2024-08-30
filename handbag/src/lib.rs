mod command;
mod workspace;
mod operation;
mod id;
mod organizer_error;
mod organizer;

pub use command::Command;
pub use workspace::{Workspace, Name as WorkspaceName};
pub use organizer_error::OrganizerError;
pub use operation::{LoadOrganizer, Load, SaveOrganizer, Save};
pub use id::Id;
pub use organizer::Organizer;

pub type OrganizerResult<T> = Result<T, OrganizerError>;
