use crate::types::shared::{Error, Result};

#[derive(Clone, Copy, PartialEq)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::str::FromStr for Id {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let inner = uuid::Uuid::parse_str(value).map_err(|err| Error::Internal(err.to_string()))?;

        Ok(Self(inner))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
