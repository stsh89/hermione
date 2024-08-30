use crate::{Organizer, OrganizerError};

pub trait Save {
    fn save(&self, projection: &Organizer) -> Result<(), OrganizerError>;
}

pub struct SaveOrganizer<S> where S: Save {
    pub saver: S
}

impl <S> SaveOrganizer<S> where S: Save {
    pub fn save(&self, projection: &Organizer) -> Result<(), OrganizerError> {
        self.saver.save(projection)
    }
}
