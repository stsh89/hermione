use crate::{Error, Organizer};

pub trait Load {
    fn load(&self) -> Result<Organizer, Error>;
}

pub struct LoadOrganizer<'a, L>
where
    L: Load,
{
    pub loader: &'a L,
}

impl<'a, L> LoadOrganizer<'a, L>
where
    L: Load,
{
    pub fn load(&self) -> Result<Organizer, Error> {
        self.loader.load()
    }
}
