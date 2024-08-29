use crate::{Projection, ProjectionError};

pub trait Load {
    fn load(&self) -> Result<Projection, ProjectionError>;
}

pub struct LoadProjection<L> where L: Load {
    pub loader: L,
}

impl<L> LoadProjection<L> where L: Load {
    pub fn load(&self) -> Result<Projection, ProjectionError> {
        self.loader.load()
    }
}
