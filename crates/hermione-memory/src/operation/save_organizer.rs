use crate::{Error, Organizer};

pub trait Save {
    fn save(&self, organizer: &Organizer) -> Result<(), Error>;
}

pub struct SaveOrganizer<'a, S>
where
    S: Save,
{
    pub saver: &'a S,
}

impl<'a, S> SaveOrganizer<'a, S>
where
    S: Save,
{
    pub fn save(&self, organizer: &Organizer) -> Result<(), Error> {
        self.saver.save(organizer)
    }
}
