use eyre::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

pub struct CollectionManager {
    path_buf: PathBuf,
}

impl CollectionManager {
    pub fn new(path_buf: PathBuf) -> Result<Self> {
        let path = Path::new(&path_buf);

        if !path.try_exists()? {
            let mut file = File::create(path)?;
            file.write_all(b"[]")?;
        }

        Ok(Self { path_buf })
    }

    pub fn read<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let file = File::open(&self.path_buf)?;
        let reader = BufReader::new(file);
        let collection = serde_json::from_reader(reader)?;

        Ok(collection)
    }

    pub fn write<S>(&self, collection: Vec<S>) -> Result<()>
    where
        S: Serialize,
    {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path_buf)?;

        serde_json::to_writer(&mut file, &collection)?;

        Ok(())
    }
}
