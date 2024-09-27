use crate::types::{
    command::Entity,
    shared::{Error, Result},
};

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
        let command = self.creator.create(command)?;

        if command.get_id().is_none() {
            return Err(Error::Internal(
                "Failed to create command: command id is not set".to_string(),
            ));
        };

        Ok(command)
    }
}