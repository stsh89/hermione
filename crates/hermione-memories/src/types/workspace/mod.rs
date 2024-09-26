mod location;
mod name;

use crate::types::{
    command,
    shared::{DateTime, Error, Id, Result},
};
pub use location::Location;
pub use name::Name;

pub struct Entity {
    commands: Vec<command::Entity>,
    id: Option<Id>,
    last_load_time: Option<DateTime>,
    location: Option<Location>,
    name: Name,
}

pub struct LoadParameters {
    pub commands: Vec<command::Entity>,
    pub id: Id,
    pub last_load_time: Option<DateTime>,
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

    pub fn rename(&mut self, name: Name) {
        self.name = name;
    }

    pub fn commands(&self) -> &[command::Entity] {
        &self.commands
    }

    pub fn id(&self) -> Result<Id> {
        self.id
            .ok_or(Error::DataLoss("Workspace Id not set".to_string()))
    }

    pub fn load(parameters: LoadParameters) -> Self {
        let LoadParameters {
            commands,
            id,
            last_load_time,
            location,
            name,
        } = parameters;

        Self {
            commands,
            id: Some(id),
            last_load_time,
            location,
            name,
        }
    }

    pub fn last_load_time(&self) -> Option<&DateTime> {
        self.last_load_time.as_ref()
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn new(parameters: NewParameters) -> Result<Self> {
        let NewParameters { name, location } = parameters;

        Ok(Self {
            commands: vec![],
            id: None,
            last_load_time: None,
            location,
            name,
        })
    }

    pub fn set_id(&mut self, id: Id) -> Result<()> {
        if self.id.is_some() {
            return Err(Error::AlreadyExists(format!(
                "Workspace with id {} already exists",
                id
            )));
        }

        self.id = Some(id);

        Ok(())
    }

    pub fn update_last_load_time(&mut self) {
        self.last_load_time = Some(DateTime::now());
    }
}
