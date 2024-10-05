use serde::{de::DeserializeOwned, Serialize};
use std::{fs, io, path};

pub struct Client {
    path_buf: path::PathBuf,
}

impl Client {
    pub fn new(path_buf: path::PathBuf) -> Result<Self, io::Error> {
        let path = path::Path::new(&path_buf);

        if !path.try_exists()? {
            let mut file = fs::File::create(path)?;

            use std::io::Write;
            file.write_all(b"[]")?;
        }

        Ok(Self { path_buf })
    }

    pub fn read_collection<T>(&self) -> Result<Vec<T>, io::Error>
    where
        T: DeserializeOwned,
    {
        let file = fs::File::open(&self.path_buf)?;
        let reader = io::BufReader::new(file);
        let collection = serde_json::from_reader(reader)?;

        Ok(collection)
    }

    pub fn write_collection<S>(&self, collection: Vec<S>) -> Result<(), io::Error>
    where
        S: Serialize,
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path_buf)?;

        serde_json::to_writer(&mut file, &collection)?;

        Ok(())
    }
}
