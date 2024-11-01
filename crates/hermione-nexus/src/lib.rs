pub mod services;

mod create_workspace;
mod failure;
mod list_workspaces;
mod update_workspace;

pub use create_workspace::*;
pub use failure::*;
pub use list_workspaces::*;
pub use update_workspace::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait StorageProvider {}
