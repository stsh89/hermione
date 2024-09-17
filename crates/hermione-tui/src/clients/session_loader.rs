use std::{
    fs::{File, OpenOptions},
    io::BufReader,
};

use crate::{
    session::{Properties, Session, SessionParameters},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SessionRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_number: Option<usize>,
}

pub struct Client {
    pub path: String,
}

impl Client {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn load(&self) -> Result<Session> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let session_record: SessionRecord = serde_json::from_reader(reader)?;

        Ok(into_session(session_record))
    }

    pub fn save(&self, session: Session) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        let session_record = into_session_record(session);

        serde_json::to_writer(&mut file, &session_record)?;

        Ok(())
    }
}

fn into_session(session_record: SessionRecord) -> Session {
    let SessionRecord { workspace_number } = session_record;

    Session::new(SessionParameters { workspace_number })
}

fn into_session_record(session: Session) -> SessionRecord {
    let Properties { workspace_number } = session.properties();

    SessionRecord { workspace_number }
}
