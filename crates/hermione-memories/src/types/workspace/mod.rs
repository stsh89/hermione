mod location;
mod name;

use crate::types::shared::{DateTime, Error, Id, Result};
pub use location::Location;
pub use name::Name;

pub struct Entity {
    id: Option<Id>,
    last_load_time: Option<DateTime>,
    location: Option<Location>,
    name: Name,
}

pub struct LoadParameters {
    pub id: Id,
    pub last_access_time: Option<DateTime>,
    pub location: Option<Location>,
    pub name: Name,
}

pub struct NewParameters {
    pub name: Name,
    pub location: Option<Location>,
}

impl Entity {
    pub fn change_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    /// # Safety
    ///
    /// It can be called only for loaded workspaces
    pub unsafe fn id(&self) -> Id {
        self.id.unwrap()
    }

    pub fn last_access_time(&self) -> Option<&DateTime> {
        self.last_load_time.as_ref()
    }

    pub fn load(parameters: LoadParameters) -> Self {
        let LoadParameters {
            id,
            last_access_time: last_load_time,
            location,
            name,
        } = parameters;

        Self {
            id: Some(id),
            last_load_time,
            location,
            name,
        }
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn get_id(&self) -> Option<Id> {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(parameters: NewParameters) -> Self {
        let NewParameters { name, location } = parameters;

        Self {
            id: None,
            last_load_time: None,
            location,
            name,
        }
    }

    pub fn rename(&mut self, name: Name) {
        self.name = name;
    }

    pub fn set_id(&mut self, id: Id) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::Internal("Workspace id is already set".to_string()));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn update_last_load_time(&mut self) {
        self.last_load_time = Some(DateTime::now());
    }
}
