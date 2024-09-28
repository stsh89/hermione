use std::ops::Deref;

use crate::types::{Error, Result};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
pub struct Id(Uuid);

impl Id {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl std::str::FromStr for Id {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let inner = Uuid::parse_str(value).map_err(|err| Error::Internal(err.to_string()))?;

        Ok(Self(inner))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Id {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
