use crate::types::{
    command::Entity,
    shared::{Error, Id, Result},
};

pub trait Create {
    fn create(&self, workspace_id: Id, command: Entity) -> Result<Entity>;
}

pub struct Operation<'a, S> {
    pub creator: &'a S,
}

impl<'a, S> Operation<'a, S>
where
    S: Create,
{
    pub fn execute(&self, workspace_id: Id, command: Entity) -> Result<Entity> {
        let command = self.creator.create(workspace_id, command)?;

        if command.id().is_err() {
            return Err(Error::Internal(
                "Failed to create command: command id is not set".to_string(),
            ));
        };

        Ok(command)
    }
}
