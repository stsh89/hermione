pub mod workspaces;

use crate::Result;
use std::path::Path;

pub struct Coordinator {
    workspaces: workspaces::Coordinator,
}

impl Coordinator {
    pub fn new(connection_path: &Path) -> Result<Self> {
        Ok(Self {
            workspaces: workspaces::Coordinator::new(connection_path)?,
        })
    }

    pub fn workspaces(&self) -> &workspaces::Coordinator {
        &self.workspaces
    }
}
