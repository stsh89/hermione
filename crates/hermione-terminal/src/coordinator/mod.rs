pub mod workspaces;

use crate::Result;
use std::path::Path;

pub struct Coordinator {
    workspaces: workspaces::Coordinator,
}

impl Coordinator {
    pub fn new(app_path: &Path) -> Result<Self> {
        Ok(Self {
            workspaces: workspaces::Coordinator::new(app_path)?,
        })
    }

    pub fn workspaces(&self) -> &workspaces::Coordinator {
        &self.workspaces
    }
}
