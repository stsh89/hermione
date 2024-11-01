pub mod services;

mod create_workspace;
mod failure;
mod update_workspace;

pub use create_workspace::*;
pub use failure::*;
pub use update_workspace::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait StorageProvider {}
