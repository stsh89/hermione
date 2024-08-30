use crate::{Organizer, OrganizerError};

pub trait Load {
    fn load(&self) -> Result<Organizer, OrganizerError>;
}

pub struct LoadOrganizer<L>
where
    L: Load,
{
    pub loader: L,
}

impl<L> LoadOrganizer<L>
where
    L: Load,
{
    pub fn load(&self) -> Result<Organizer, OrganizerError> {
        self.loader.load()
    }
}
