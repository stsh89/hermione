use crate::types::{
    shared::{Error, Result},
    workspace::Entity,
};

pub trait Track {
    fn track(&self, workspace: Entity) -> Result<Entity>;
}

pub struct Operation<'a, T> {
    pub tracker: &'a T,
}

impl<'a, T> Operation<'a, T>
where
    T: Track,
{
    pub fn execute(&self, workspace: Entity) -> Result<Entity> {
        let time = workspace.last_access_time().cloned();

        let command = self.tracker.track(workspace)?;
        let error_message = "Failed to track workspace access time".to_string();

        if let Some(new_time) = command.last_access_time() {
            if let Some(time) = time {
                if time >= *new_time {
                    return Err(Error::Internal(error_message));
                }
            }
        } else {
            return Err(Error::Internal(error_message));
        }

        Ok(command)
    }
}
