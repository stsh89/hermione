use crate::{entities::command::Entity, Error, Result};

pub trait Create {
    fn create(&self, command: Entity) -> Result<Entity>;
}

pub struct Operation<'a, S> {
    pub creator: &'a S,
}

impl<'a, S> Operation<'a, S>
where
    S: Create,
{
    pub fn execute(&self, command: Entity) -> Result<Entity> {
        if command.id().is_some() {
            return Err(Error::FailedPrecondition(
                "Command id is already set".to_string(),
            ));
        }

        let command = self.creator.create(command)?;

        if command.id().is_none() {
            return Err(Error::Internal(
                "Failed to create command: command id is not set".to_string(),
            ));
        };

        Ok(command)
    }
}
