use crate::types::{
    shared::{Error, Result},
    workspace::Entity,
};

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

        if workspace.get_id().is_none() {
            return Err(Error::Internal(
                "Failed to create workspace: workspace id is not set".to_string(),
            ));
        };

        Ok(workspace)
    }
}
