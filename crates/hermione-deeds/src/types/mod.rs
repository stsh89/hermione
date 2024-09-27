pub mod command;
pub mod workspace;

pub type Result<T> = std::result::Result<T, eyre::Report>;
