use crate::types::{
    command::Entity,
    shared::{Error, Result},
};

pub trait Track {
    fn track(&self, command: Entity) -> Result<Entity>;
}

pub struct Operation<'a, T> {
    pub tracker: &'a T,
}

impl<'a, T> Operation<'a, T>
where
    T: Track,
{
    pub fn execute(&self, command: Entity) -> Result<Entity> {
        let time = command.last_execute_time().cloned();

        let command = self.tracker.track(command)?;
        let error_message = "Failed to track command execution time".to_string();

        if let Some(new_time) = command.last_execute_time() {
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
