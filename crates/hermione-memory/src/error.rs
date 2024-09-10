#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0} not found")]
    NotFound(String),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}
