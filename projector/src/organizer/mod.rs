mod static_client;

use static_client::StaticClient;

use crate::AppResult;

pub struct OrganizerCLient {
    inner: StaticClient,
}

pub struct Workspace {
    pub name: String,
    pub commands: Vec<Command>,
}

pub struct Command {
    pub name: String,
    pub program: String,
}

impl OrganizerCLient {
    pub fn initialize() -> Self {
        Self {
            inner: StaticClient::new(),
        }
    }

    pub fn workspaces(&self) -> AppResult<Vec<Workspace>> {
        let organizer = self.inner.load_organizer()?;

        let workspaces = organizer
            .workspaces()
            .iter()
            .map(|w| Workspace {
                name: w.name().to_string(),
                commands: w
                    .commands()
                    .iter()
                    .map(|c| Command {
                        name: c.name().to_string(),
                        program: c.program().to_string(),
                    })
                    .collect(),
            })
            .collect();

        Ok(workspaces)
    }
}
