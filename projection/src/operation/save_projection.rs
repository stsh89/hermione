use crate::{Projection, ProjectionError};

pub trait Save {
    fn save(&self, projection: &Projection) -> Result<(), ProjectionError>;
}

pub struct SaveProjection<S> where S: Save {
    pub saver: S
}

impl <S> SaveProjection<S> where S: Save {
    pub fn save(&self, projection: &Projection) -> Result<(), ProjectionError> {
        self.saver.save(projection)
    }
}
