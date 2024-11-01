pub mod definitions;
pub mod operations;
pub mod services;

mod failure;

pub use failure::Error;

pub type Result<T> = std::result::Result<T, Error>;
