mod backup_credentials;
mod commands;
mod workspaces;

pub use backup_credentials::*;
pub use commands::*;
pub use workspaces::*;

pub enum OptionalValue<T> {
    Null,
    Value(T),
}

impl<T> From<OptionalValue<T>> for Option<T> {
    fn from(value: OptionalValue<T>) -> Self {
        match value {
            OptionalValue::Null => None,
            OptionalValue::Value(value) => Some(value),
        }
    }
}

impl<T> From<Option<T>> for OptionalValue<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            None => Self::Null,
            Some(value) => Self::Value(value),
        }
    }
}
