use crate::Result;
use std::collections::HashSet;

const RESTORE_PROPERTIES_NUMBER: usize = 1;

pub struct Session {
    /// Workspace ID of the current session
    workspace_id: Option<usize>,

    /// State of the session
    state: State,

    properties: HashSet<RestoreProperty>,
}

pub struct SessionParameters {
    /// Workspace ID of the last session
    pub workspace_id: Option<usize>,
}

enum State {
    ReadOnly,
    WriteOnly,
}

pub struct Properties {
    pub workspace_id: Option<usize>,
}

#[derive(Eq, Hash, PartialEq)]
enum RestoreProperty {
    WorkspaceId,
}

impl Session {
    pub fn get_workspace_id(&mut self) -> Result<Option<usize>> {
        if self.write_only() {
            return Err(anyhow::anyhow!("attempt to read write-only session"));
        }

        self.restore_property(RestoreProperty::WorkspaceId);

        Ok(self.workspace_id)
    }

    pub fn new(parameters: SessionParameters) -> Self {
        let SessionParameters { workspace_id } = parameters;

        Self {
            properties: HashSet::new(),
            state: State::ReadOnly,
            workspace_id,
        }
    }

    pub fn properties(self) -> Properties {
        Properties {
            workspace_id: self.workspace_id,
        }
    }

    pub fn read_only(&self) -> bool {
        self.state.is_read_only()
    }

    fn restore_property(&mut self, property: RestoreProperty) {
        self.properties.insert(property);

        if self.properties.len() == RESTORE_PROPERTIES_NUMBER {
            self.state = State::WriteOnly;
        }
    }

    pub fn set_workspace_id(&mut self, id: Option<usize>) -> Result<()> {
        if self.read_only() {
            return Err(anyhow::anyhow!("attempt to modify read-only session"));
        }

        self.workspace_id = id;

        Ok(())
    }

    fn write_only(&self) -> bool {
        self.state.is_write_only()
    }
}

impl State {
    fn is_write_only(&self) -> bool {
        matches!(self, State::WriteOnly)
    }

    fn is_read_only(&self) -> bool {
        matches!(self, State::ReadOnly)
    }
}
