pub mod workspaces;

use crate::Result;
use hermione_coordinator::Connection;
use std::{path::Path, rc::Rc};

pub struct Coordinator {
    workspaces: workspaces::Coordinator,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        let connection = Rc::new(Connection::open(app_path)?);

        Ok(Self {
            workspaces: workspaces::Coordinator::new(connection),
        })
    }

    pub fn workspaces(&self) -> &workspaces::Coordinator {
        &self.workspaces
    }
}
