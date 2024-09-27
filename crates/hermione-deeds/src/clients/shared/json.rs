use std::{
    fs::{File, OpenOptions},
    io::BufReader,
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

pub struct Client {
    path: PathBuf,
}

impl Client {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read<T>(&self) -> Result<Vec<T>, eyre::Report>
    where
        T: DeserializeOwned,
    {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let records = serde_json::from_reader(reader)?;

        Ok(records)
    }

    pub fn save<S>(&self, records: Vec<S>) -> Result<(), eyre::Report>
    where
        S: Serialize,
    {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        serde_json::to_writer(&mut file, &records)?;

        Ok(())
    }
}
