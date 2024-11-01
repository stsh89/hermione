pub mod services;

mod create_workspace;
mod failure;

pub use failure::*;
pub use create_workspace::*;

pub type Result<T> = std::result::Result<T, Error>;

pub trait StorageProvider {}
