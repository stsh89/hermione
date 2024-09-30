use crate::{entities::workspace::Entity, Error, Result};

pub trait Create {
    fn create(&self, workspace: Entity) -> Result<Entity>;
}

pub struct Operation<'a, S> {
    pub creator: &'a S,
}

impl<'a, S> Operation<'a, S>
where
    S: Create,
{
    pub fn execute(&self, workspace: Entity) -> Result<Entity> {
        let workspace = self.creator.create(workspace)?;

        if workspace.id().is_none() {
            return Err(Error::Internal(
                "Failed to create workspace: workspace id is not set".to_string(),
            ));
        };

        Ok(workspace)
    }
}
