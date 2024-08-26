use crate::CoreError;
use std::ops::Deref;

pub struct Number(u32);

impl Number {
    pub fn new(value: u32) -> Result<Self, CoreError> {
        if value == 0 {
            return Err(CoreError::InvariantViolation(
                "Workspace number can't be equal to zero",
            ));
        }

        Ok(Self(value))
    }
}

impl Deref for Number {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
