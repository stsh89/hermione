use crate::Result;
use std::collections::HashSet;

const RESTORE_PROPERTIES_NUMBER: usize = 1;

pub struct Session {
    /// Workspace number of the current session
    workspace_number: Option<usize>,

    /// State of the session
    state: State,

    properties: HashSet<RestoreProperty>,
}

pub struct SessionParameters {
    /// Workspace number of the last session
    pub workspace_number: Option<usize>,
}

enum State {
    ReadOnly,
    WriteOnly,
}

pub struct Properties {
    pub workspace_number: Option<usize>,
}

#[derive(Eq, Hash, PartialEq)]
enum RestoreProperty {
    WorkspaceNumber,
}

impl Session {
    pub fn get_workspace_number(&mut self) -> Result<Option<usize>> {
        if self.write_only() {
            return Err(anyhow::anyhow!("attempt to read write-only session"));
        }

        self.restore_property(RestoreProperty::WorkspaceNumber);

        Ok(self.workspace_number)
    }

    pub fn new(parameters: SessionParameters) -> Self {
        let SessionParameters { workspace_number } = parameters;

        Self {
            properties: HashSet::new(),
            state: State::ReadOnly,
            workspace_number,
        }
    }

    pub fn properties(self) -> Properties {
        Properties {
            workspace_number: self.workspace_number,
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

    pub fn set_workspace_number(&mut self, number: Option<usize>) -> Result<()> {
        if self.read_only() {
            return Err(anyhow::anyhow!("attempt to modify read-only session"));
        }

        self.workspace_number = number;

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
