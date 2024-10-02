use crate::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

pub struct CollectionManager {
    path: PathBuf,
}

impl CollectionManager {
    pub fn new(path: PathBuf) -> Result<Self> {
        let manager = Self { path };

        manager.prepare_collection()?;

        Ok(manager)
    }

    pub fn read_collection<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let collection = serde_json::from_reader(reader)?;

        Ok(collection)
    }

    pub fn write_collection<S>(&self, collection: Vec<S>) -> Result<()>
    where
        S: Serialize,
    {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        serde_json::to_writer(&mut file, &collection)?;

        Ok(())
    }

    pub fn prepare_collection(&self) -> Result<()> {
        if AsRef::<Path>::as_ref(&self.path).try_exists()? {
            return Ok(());
        }

        let mut file = std::fs::File::create(&self.path)?;
        file.write_all(b"[]")?;

        Ok(())
    }
}
